# alluno-hinpud-sys

Rust bindings for the Alluno Hardware Input Driver (AllunoHInpuD).

Provides kernel-level hardware input emulation.

## Requirements

- Windows 10/11
- AllunoHInpuD driver installed (from [alluno-windows-drivers](https://github.com/alluno-io/alluno-windows-drivers))

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
alluno-hinpud-sys = "1.0"
```

```rust
use alluno_hinpud_sys::*;

// Keyboard
let kbd = AllunoHinpud::open_keyboard().expect("driver not installed");
kbd.send_key(scan_code::A).unwrap();           // press + release
kbd.press_key(scan_code::LEFT_SHIFT).unwrap();  // hold shift
kbd.send_key(scan_code::A).unwrap();            // Shift+A
kbd.release_key(scan_code::LEFT_SHIFT).unwrap();

// Mouse
let mou = AllunoHinpud::open_mouse().expect("driver not installed");
mou.send_move(10, 10, false).unwrap();                             // relative move
mou.send_move(32767, 32767, true).unwrap();                        // absolute move (center)
mou.send_button(mouse_button_flags::LEFT_BUTTON_DOWN).unwrap();    // left click down
mou.send_button(mouse_button_flags::LEFT_BUTTON_UP).unwrap();      // left click up
```

## API

### Keyboard (`AllunoHinpud::open_keyboard()`)

| Method | Description |
|---|---|
| `send_key(scan_code)` | Press + release a key |
| `press_key(scan_code)` | Press a key (key down) |
| `release_key(scan_code)` | Release a key (key up) |
| `send_key_raw(scan_code, flags)` | Send raw keyboard event with custom flags |

### Mouse (`AllunoHinpud::open_mouse()`)

| Method | Description |
|---|---|
| `send_move(x, y, absolute)` | Move mouse (relative or absolute 0-65535) |
| `send_button(button_flags)` | Send mouse button event |
| `send_raw(data)` | Send raw input data bytes |

### Constants

| Module | Description |
|---|---|
| `scan_code::*` | PS/2 scan codes (A-Z, F1-F12, modifiers, etc.) |
| `key_flags::*` | Key event flags (KEY_MAKE, BREAK, E0, E1) |
| `mouse_button_flags::*` | Mouse buttons (LEFT/RIGHT/MIDDLE DOWN/UP, WHEEL) |
| `mouse_move_flags::*` | Mouse move modes (MOVE_RELATIVE, MOVE_ABSOLUTE) |

## Testing

Requires the AllunoHInpuD driver to be installed.

```sh
# Run integration tests
cargo test -- --nocapture

# Run the test binary
cargo run --release --bin alluno-hinpud-test -- --driver
```

## License

[MIT](LICENSE)
