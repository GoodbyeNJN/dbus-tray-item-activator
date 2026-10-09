use clap::{Parser, Subcommand};

#[derive(Debug, Default, clap::Args)]
pub struct Args {
    /// Tray item ID.
    #[arg(long, global = true)]
    pub id: Option<String>,

    /// Tray item category.
    #[arg(long, global = true)]
    pub category: Option<String>,

    /// Tray item status.
    #[arg(long, global = true)]
    pub status: Option<String>,

    /// Tray item title.
    #[arg(long, global = true)]
    pub title: Option<String>,

    /// Tray item tooltip.
    #[arg(long, global = true)]
    pub tooltip: Option<String>,

    /// Process ID.
    #[arg(long, global = true)]
    pub pid: Option<u32>,

    /// Program name.
    #[arg(long, global = true)]
    pub program: Option<String>,

    /// Process command line.
    #[arg(long, global = true)]
    pub cmdline: Option<String>,

    /// D-Bus service name.
    #[arg(long, global = true)]
    pub dbus_service: Option<String>,

    /// D-Bus object path.
    #[arg(long, global = true)]
    pub dbus_path: Option<String>,
}

impl Args {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.category.is_none()
            && self.status.is_none()
            && self.title.is_none()
            && self.tooltip.is_none()
            && self.pid.is_none()
            && self.program.is_none()
            && self.cmdline.is_none()
            && self.dbus_service.is_none()
            && self.dbus_path.is_none()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Activate a tray item matching the specified properties.
    Activate,

    /// List registered tray items and their properties.
    List {
        #[arg(short, long, default_value_t = false)]
        json: bool,
    },
}

#[derive(Debug, Parser)]
#[command(
    about="A command-line tool for listing and activating StatusNotifier tray items through the session D-Bus.",
    long_about = None,
    version,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub args: Args,
}
