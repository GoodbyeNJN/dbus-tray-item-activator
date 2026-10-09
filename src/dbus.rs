use std::fs;

use anyhow::{Context, Result, bail};
use zbus::blocking::Connection;
use zbus::blocking::fdo::DBusProxy;
use zbus::proxy;

use crate::tray_item::TrayItem;

#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/StatusNotifierWatcher",
    interface = "org.kde.StatusNotifierWatcher"
)]
trait StatusNotifierWatcher {
    fn get_registered_items(&self) -> zbus::Result<Vec<String>>;
}

#[proxy(interface = "org.kde.StatusNotifierItem")]
trait StatusNotifierItem {
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn category(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn status(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn title(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn tool_tip(&self) -> zbus::Result<(String, Vec<(i32, i32, Vec<u8>)>, String, String)>;

    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;
}

#[derive(Debug)]
struct StatusNotifier {
    service: String,
    path: String,
}

#[derive(Debug)]
struct Process {
    pid: u32,
    program: Option<String>,
    cmdline: Option<String>,
}

#[derive(Debug)]
struct KnownProps {
    id: String,
    category: Option<String>,
    status: Option<String>,
    title: Option<String>,
    tooltip: Option<String>,
}

pub struct Dbus {
    connection: Connection,
}

impl Dbus {
    pub fn connect() -> Result<Self> {
        let connection = Connection::session()?;

        Ok(Self { connection })
    }

    pub fn get_tray_items(&self) -> Result<Vec<TrayItem>> {
        let notifiers = self.get_status_notifiers()?;

        let mut tray_items = vec![];
        for notifier in notifiers {
            let process = self.get_process_for_notifier(&notifier).with_context(|| {
                format!(
                    "Could not read process information for tray item at {}{}.",
                    notifier.service, notifier.path
                )
            })?;
            let known_props = self
                .get_known_props_for_notifier(&notifier)
                .with_context(|| {
                    format!(
                        "Could not read properties for tray item at {}{}.",
                        notifier.service, notifier.path
                    )
                })?;

            tray_items.push(TrayItem {
                id: known_props.id,
                category: known_props.category,
                status: known_props.status,
                title: known_props.title,
                tooltip: known_props.tooltip,
                pid: process.pid,
                program: process.program,
                cmdline: process.cmdline,
                dbus_service: notifier.service,
                dbus_path: notifier.path,
            });
        }
        tray_items.sort_by(|a, b| {
            (a.pid, &a.dbus_service, &a.dbus_path).cmp(&(b.pid, &b.dbus_service, &b.dbus_path))
        });

        Ok(tray_items)
    }

    pub fn activate_tray_item(&self, tray_item: &TrayItem) -> Result<()> {
        let proxy = StatusNotifierItemProxyBlocking::builder(&self.connection)
            .destination(tray_item.dbus_service.as_str())?
            .path(tray_item.dbus_path.as_str())?
            .build()?;

        proxy.activate(0, 0)?;

        Ok(())
    }

    fn get_status_notifiers(&self) -> Result<Vec<StatusNotifier>> {
        let proxy = StatusNotifierWatcherProxyBlocking::new(&self.connection)?;

        let reply = proxy.get_registered_items()?;

        let mut items = vec![];
        for item in &reply {
            let Some((service, path)) = item.split_once('/') else {
                bail!("The StatusNotifier item name has an invalid format: {item}.")
            };

            items.push(StatusNotifier {
                service: service.to_owned(),
                path: format!("/{path}"),
            });
        }

        Ok(items)
    }

    fn get_process_for_notifier(&self, notifier: &StatusNotifier) -> Result<Process> {
        let proxy = DBusProxy::new(&self.connection)?;

        let pid = proxy.get_connection_unix_process_id(notifier.service.as_str().try_into()?);
        let Ok(pid) = pid else {
            bail!(
                "Could not retrieve the process ID for tray item service '{}'.",
                notifier.service
            )
        };

        let program = fs::read_link(format!("/proc/{pid}/exe"))
            .map(|path| path.to_string_lossy().to_string());

        let cmdline = fs::read(format!("/proc/{pid}/cmdline")).map(|bytes| {
            bytes
                .split(|&b| b == 0) // split by null byte
                .filter(|slice| !slice.is_empty())
                .map(|slice| String::from_utf8_lossy(slice).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        });

        Ok(Process {
            pid,
            program: program.ok(),
            cmdline: cmdline.ok(),
        })
    }

    fn get_known_props_for_notifier(&self, notifier: &StatusNotifier) -> Result<KnownProps> {
        let proxy = StatusNotifierItemProxyBlocking::builder(&self.connection)
            .destination(notifier.service.as_str())?
            .path(notifier.path.as_str())?
            .build()?;

        let Ok(id) = proxy.id() else {
            bail!(
                "Could not retrieve the ID for tray item service '{}'.",
                notifier.service
            )
        };

        Ok(KnownProps {
            id,
            category: proxy.category().ok(),
            status: proxy.status().ok(),
            title: proxy.title().ok(),
            tooltip: proxy
                .tool_tip()
                .map(|(_, _, title, description)| format!("{title}: {description}"))
                .ok(),
        })
    }

    // fn get_all_props_for_item(&self, tray_item: &TrayItem) -> Result<BTreeMap<String, String>> {
    //     let proxy = PropertiesProxy::builder(&self.connection)
    //         .destination(tray_item.service.as_str())
    //         .into_diagnostic()?
    //         .path(tray_item.path.as_str())
    //         .into_diagnostic()?
    //         .build()
    //         .into_diagnostic()?;

    //     let props = proxy
    //         .get_all("org.kde.StatusNotifierItem".try_into().into_diagnostic()?)
    //         .into_diagnostic()?;

    //     let mut ordered = BTreeMap::new();
    //     for (key, value) in &props {
    //         ordered.insert(key.to_owned(), Self::format_value_sanitized(value));
    //     }

    //     Ok(ordered)
    // }

    // fn is_printable_str(str: &str) -> bool {
    //     str.chars()
    //         .all(|char| !char.is_control() || char == '\n' || char == '\t' || char == '\r')
    // }

    // fn format_value_sanitized(value: &Value) -> String {
    //     match value {
    //         Value::Str(str) => {
    //             let text = str.as_str();
    //             if Self::is_printable_str(text) {
    //                 format!("\"{text}\"")
    //             } else {
    //                 format!("<non-printable str, {} chars>", text.len())
    //             }
    //         }

    //         Value::Array(array) => {
    //             if array.element_signature() != "y" {
    //                 let items: Vec<String> =
    //                     array.iter().map(Self::format_value_sanitized).collect();

    //                 return format!("[{}]", items.join(", "));
    //             }

    //             let len = array.len();
    //             let bytes: Vec<u8> = array
    //                 .iter()
    //                 .filter_map(|v| match v {
    //                     Value::U8(byte) => Some(*byte),
    //                     _ => None,
    //                 })
    //                 .collect();

    //             if let Ok(str) = std::str::from_utf8(&bytes) {
    //                 if Self::is_printable_str(str) && !str.is_empty() {
    //                     return format!("\"{str}\"");
    //                 }
    //             }

    //             return format!("<binary byte array, {len} bytes>");
    //         }

    //         Value::Dict(dict) => {
    //             let mut entries = Vec::new();
    //             for (k, v) in dict.iter() {
    //                 entries.push(format!(
    //                     "{}: {}",
    //                     Self::format_value_sanitized(k),
    //                     Self::format_value_sanitized(v)
    //                 ));
    //             }

    //             format!("{{{}}}", entries.join(", "))
    //         }

    //         Value::Value(inner) => Self::format_value_sanitized(inner),

    //         other => format!("{other:?}"),
    //     }
    // }
}
