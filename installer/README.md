# Interception Driver Installer

This crate provides a library for installing and uninstalling the Interception driver on Windows systems.
It also includes a command-line installer that is built on top of this library, similar to what is provided by the
official Interception project.
It supports automatic detection of Windows versions and architectures, installs the appropriate
driver files, configures necessary registry entries, and allows for clean uninstallation.

## CLI Usage

> [!IMPORTANT]
> This installer requires administrator privileges to modify system files and registry entries.

> [!IMPORTANT]
> After installation or uninstallation, a system reboot is required for the changes to take effect.

### Installation

```cmd
interception-installer install
```

### Uninstallation

```cmd
interception-installer uninstall
```

## Supported Platforms

By default, Windows 7 and later running on x86 and x86_64 architectures are supported.

Support for older Windows versions and other niche architectures is available via the `legacy-platforms` feature flag.
However, due to limited support from the Rust compiler on these platforms, you might run into issues when building for
them.

## License

This project is licensed under the LGPL-3.0-only License. See the [LICENSE](../LICENSE) file for details.
