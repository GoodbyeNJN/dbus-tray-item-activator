use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TrayItem {
    pub id: String,
    pub category: Option<String>,
    pub status: Option<String>,
    pub title: Option<String>,
    pub tooltip: Option<String>,
    pub pid: u32,
    pub program: Option<String>,
    pub cmdline: Option<String>,
    #[serde(rename = "dbus-service")]
    pub dbus_service: String,
    #[serde(rename = "dbus-path")]
    pub dbus_path: String,
}

impl std::fmt::Display for TrayItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ALIGN_WIDTH: usize = 12 + 1;
        const UNAVAILABLE: &str = "<unavailable>";

        write!(f, "{:ALIGN_WIDTH$}: {:?}\n", "id", self.id)?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "category",
            self.category.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "status",
            self.status.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "title",
            self.title.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "tooltip",
            self.tooltip.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(f, "{:ALIGN_WIDTH$}: {:?}\n", "pid", self.pid)?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "program",
            self.program.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "cmdline",
            self.cmdline.as_deref().unwrap_or(UNAVAILABLE)
        )?;
        write!(
            f,
            "{:ALIGN_WIDTH$}: {:?}\n",
            "dbus-service", self.dbus_service
        )?;
        write!(f, "{:ALIGN_WIDTH$}: {:?}\n", "dbus-path", self.dbus_path)
    }
}
