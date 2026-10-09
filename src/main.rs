use anyhow::{Result, ensure};
use clap::Parser;

mod cli;
mod dbus;
mod tray_item;

use crate::cli::{Args, Cli, Command};
use crate::dbus::Dbus;
use crate::tray_item::TrayItem;

fn format_list<'a, I: IntoIterator<Item = &'a TrayItem>>(items: I) -> String {
    let mut result = String::new();
    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            result.push('\n');
        }

        result.push_str(&format!("Tray item #{index}\n"));
        result.push_str(&format!("{}\n", "-".repeat(20)));
        result.push_str(&format!("{}", item));
    }

    result
}

fn filter_list_by_args<'a, I: IntoIterator<Item = &'a TrayItem>>(
    items: I,
    args: &Args,
) -> Vec<&'a TrayItem> {
    items
        .into_iter()
        .filter(|item| {
            if let (Some(item_id), Some(args_id)) = (Some(&item.id), &args.id) {
                if item_id != args_id {
                    return false;
                }
            }
            if let (Some(item_category), Some(args_category)) = (&item.category, &args.category) {
                if item_category != args_category {
                    return false;
                }
            }
            if let (Some(item_status), Some(args_status)) = (&item.status, &args.status) {
                if item_status != args_status {
                    return false;
                }
            }
            if let (Some(item_title), Some(args_title)) = (&item.title, &args.title) {
                if item_title != args_title {
                    return false;
                }
            }
            if let (Some(item_tooltip), Some(args_tooltip)) = (&item.tooltip, &args.tooltip) {
                if item_tooltip != args_tooltip {
                    return false;
                }
            }
            if let (Some(item_pid), Some(args_pid)) = (Some(&item.pid), &args.pid) {
                if item_pid != args_pid {
                    return false;
                }
            }
            if let (Some(item_program), Some(args_program)) = (&item.program, &args.program) {
                if item_program != args_program {
                    return false;
                }
            }
            if let (Some(item_cmdline), Some(args_cmdline)) = (&item.cmdline, &args.cmdline) {
                if item_cmdline != args_cmdline {
                    return false;
                }
            }
            if let (Some(item_dbus_service), Some(args_dbus_service)) =
                (Some(&item.dbus_service), &args.dbus_service)
            {
                if item_dbus_service != args_dbus_service {
                    return false;
                }
            }
            if let (Some(item_dbus_path), Some(args_dbus_path)) =
                (Some(&item.dbus_path), &args.dbus_path)
            {
                if item_dbus_path != args_dbus_path {
                    return false;
                }
            }

            true
        })
        .collect()
}

fn activate(args: Args) -> Result<()> {
    ensure!(
        !args.is_empty(),
        "Provide at least one option to identify a tray item."
    );

    let dbus = Dbus::connect()?;
    let tray_items = Dbus::connect()?.get_tray_items()?;
    ensure!(!tray_items.is_empty(), "No tray items are registered.");

    let matched_tray_items = filter_list_by_args(&tray_items, &args);
    let len = matched_tray_items.len();
    ensure!(len > 0, "No tray items match the specified criteria.");
    ensure!(
        len == 1,
        "Multiple tray items match the specified criteria. Refine the options to select one:\n\n{}",
        &format_list(matched_tray_items)
    );

    dbus.activate_tray_item(&matched_tray_items[0])
}

fn list(args: Args, json: bool) -> Result<()> {
    let dbus = Dbus::connect()?;
    let tray_items = dbus.get_tray_items()?;
    ensure!(!tray_items.is_empty(), "No tray items are registered.");

    let matched_tray_items = if args.is_empty() {
        tray_items.iter().collect::<Vec<_>>()
    } else {
        let matched = filter_list_by_args(&tray_items, &args);
        let len = matched.len();
        ensure!(len > 0, "No tray items match the specified criteria.");

        matched
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&matched_tray_items)?);
    } else {
        println!("{}", format_list(matched_tray_items));
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Activate) => activate(cli.args),
        Some(Command::List { json }) => list(cli.args, json),
        None => activate(cli.args),
    }
}
