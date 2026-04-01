# Interception

A Rust port of the [Interception library](https://github.com/oblitum/Interception) for intercepting keyboard and mouse
input on Windows systems.

## Overview

Interception provides a low-level interface for intercepting and modifying keyboard and mouse input at the driver level
on Windows. This Rust implementation maintains full compatibility with the original C library while providing modern
safety guarantees.

## Features

- **Low-level input interception** - Intercept keyboard and mouse events at the driver level
- **Real-time input modification** - Modify or block input events in real-time
- **No C library dependency** - Communicates directly with the driver via `DeviceIoControl`, unlike [interception-rs](https://github.com/bozbez/interception-rs) which binds to the C library
- **Safe Rust API** - Memory-safe wrapper around Windows APIs
- **Driver installer included** - Self-contained installer for required Windows drivers

## Components

### Main Library

The core library provides functions for:

- Setting up device contexts and filters
- Receiving keyboard and mouse events
- Sending modified events back to the system
- Managing device priorities and filters

### Driver Installer

A comprehensive installer utility that handles driver installation and setup automatically:

```bash
# Install Interception drivers
interception-installer install

# Remove Interception drivers  
interception-installer uninstall
```

## Requirements

- Windows 7 or later (Windows XP/Vista supported via the `legacy-platforms` feature flag)
- Administrator privileges for driver installation
- System reboot required after driver installation

## Installation

1. Download the latest release
2. Run the installer with administrator privileges:
   ```bash
   interception-installer install
   ```
3. Reboot your system
4. You can now use applications that depend on Interception

## Usage Example

```rust
use interception::{Device, FILTER_KEY_ALL, Interception, KeyStroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interception = Interception::new()?;

    // Enable keyboard interception on all keyboard devices
    for device in interception.devices_mut() {
        if let Device::Keyboard(keyboard) = device {
            keyboard.set_filter(FILTER_KEY_ALL)?;
        }
    }

    let mut strokes = [KeyStroke::default(); 10];

    loop {
        let device_index = interception.wait_index(None)?;
        let device = &mut interception.devices_mut()[device_index];

        if let Device::Keyboard(keyboard) = device {
            let received = keyboard.receive(&mut strokes)?;

            // ... process strokes here ...

            // Forward strokes so they reach the application
            keyboard.send(received)?;
        }
    }
}
```

## License

This project is licensed under the LGPL-3.0-only License. See the [LICENSE](LICENSE) file for details.

## Reference

Based on the original [Interception library](https://github.com/oblitum/Interception) by Francisco Lopes.
