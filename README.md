# Interception

A Rust port of the [Interception library](https://github.com/oblitum/Interception) for intercepting keyboard and mouse
input on Windows systems.

## Features

- **Low-level input interception** - Intercept keyboard and mouse events at the driver level
- **Real-time input modification** - Modify or block input events in real-time
- **No C library dependency** - Communicates directly with the driver via `DeviceIoControl`, unlike [interception-rs](https://github.com/bozbez/interception-rs) which binds to the C library
- **Safe Rust API** - Memory-safe wrapper around Windows APIs
- **Driver installer included** - Self-contained installer for required Windows drivers

## Requirements

- Windows 7 or later (Windows XP/Vista supported via the `legacy-platforms` feature flag)
- Administrator privileges for driver installation
- System reboot required after driver installation

## Installation

1. Download the latest `interception-installer` binary from the [releases page](../../releases)
2. Run the installer with administrator privileges:
   ```bash
   interception-installer install
   ```
3. Reboot your system

To uninstall the drivers:
```bash
interception-installer uninstall
```

## Usage

```rust
use interception::{Device, FILTER_KEY_ALL, Interception, KeyStroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interception = Interception::new()?;

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
            keyboard.send(received)?;
        }
    }
}
```

For more examples, see the [`examples/`](examples/) directory:
- [`escape_blocker.rs`](examples/escape_blocker.rs) — block the Escape key
- [`keylogger.rs`](examples/keylogger.rs) — log all key events
- [`mouse_capture.rs`](examples/mouse_capture.rs) — capture mouse input

## License

This project is licensed under the LGPL-3.0-only License. See the [LICENSE](LICENSE) file for details.

## Reference

Based on the original [Interception library](https://github.com/oblitum/Interception) by Francisco Lopes.
