# dbus-tray-item-activator

A command-line tool for listing and activating StatusNotifier tray items through the session D-Bus.

## Usage

If no subcommand is given, the tool runs `activate`. Pass one or more item properties to identify the tray item:

```sh
dbus-tray-item-activator --id "my-tray-item"
```

The `activate` subcommand is also available explicitly:

```sh
dbus-tray-item-activator activate --title "My Tray Item"
```

Use `list` to display registered tray items and their available properties. You can optionally filter the results by passing one or more item properties:

```sh
dbus-tray-item-activator list
dbus-tray-item-activator list --id "my-tray-item"
dbus-tray-item-activator list --category ApplicationStatus --status Active
```

By default, `list` prints a human-readable list. Use `--json` (or `-j`) to print the tray items as pretty-printed JSON:

```sh
dbus-tray-item-activator list --json
# or
dbus-tray-item-activator list -j
```

You can combine JSON output with filters:

```sh
dbus-tray-item-activator list --title "My Tray Item" --json
```

## Filtering options

These options are available for both `activate` and `list`. Multiple options can be combined to narrow the results:

| Option                          | Description          |
| ------------------------------- | -------------------- |
| `--id <ID>`                     | Tray item ID         |
| `--category <CATEGORY>`         | Tray item category   |
| `--status <STATUS>`             | Tray item status     |
| `--title <TITLE>`               | Tray item title      |
| `--tooltip <TOOLTIP>`           | Tray item tooltip    |
| `--pid <PID>`                   | Process ID           |
| `--program <PROGRAM>`           | Program name         |
| `--cmdline <CMDLINE>`           | Process command line |
| `--dbus-service <DBUS_SERVICE>` | D-Bus service name   |
| `--dbus-path <DBUS_PATH>`       | D-Bus object path    |

For `activate`, at least one option is required, and the specified properties must identify exactly one registered tray item. For `list`, all options are optional; when provided, they filter the displayed items.

Run `dbus-tray-item-activator --help`, `dbus-tray-item-activator activate --help`, or `dbus-tray-item-activator list --help` for command-line help.
