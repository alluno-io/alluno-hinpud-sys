//! Rust bindings for Alluno Hardware Input Driver (AllunoHInpuD)
//!
//! Provides kernel-level hardware input emulation (no INJECTED flag) via the
//! AllunoHInpuD KMDF filter driver.
//!
//! # Example
//! ```no_run
//! use alluno_hinpud_sys::*;
//!
//! let kbd = AllunoHinpudKeyboard::new().expect("keyboard driver not installed");
//! kbd.send_key(scan_code::A).unwrap();                               // press + release
//! kbd.press_key(scan_code::LEFT_SHIFT).unwrap();                     // hold shift
//! kbd.send_key(scan_code::A).unwrap();                               // Shift+A
//! kbd.release_key(scan_code::LEFT_SHIFT).unwrap();
//!
//! let mou = AllunoHinpudMouse::new().expect("mouse driver not installed");
//! mou.send_move(10, 10, false).unwrap();                             // relative move
//! mou.send_move(32767, 32767, true).unwrap();                        // absolute move (center)
//! mou.send_button(mouse_button_flags::LEFT_BUTTON_DOWN).unwrap();    // left click down
//! mou.send_button(mouse_button_flags::LEFT_BUTTON_UP).unwrap();      // left click up
//! mou.send_wheel(120).unwrap();                                       // scroll up one notch
//! mou.send_hwheel(-120).unwrap();                                     // scroll left one notch
//! ```

use std::ffi::c_void;
use std::mem::size_of;

// ============================================================================
// IOCTL — must match Driver.h
// ============================================================================

// CTL_CODE(FILE_DEVICE_KEYBOARD=0xB, 0x820, METHOD_BUFFERED=0, FILE_ANY_ACCESS=0)
const IOCTL_KEYBOARD_SEND: u32 = (0x000B << 16) | (0x820 << 2);
// CTL_CODE(FILE_DEVICE_MOUSE=0xF, 0x820, METHOD_BUFFERED=0, FILE_ANY_ACCESS=0)
const IOCTL_MOUSE_SEND: u32 = (0x000F << 16) | (0x820 << 2);

// ============================================================================
// Wire structs — must match kernel KEYBOARD_INPUT_DATA / MOUSE_INPUT_DATA
// ============================================================================

/// Keyboard input data (matches kernel KEYBOARD_INPUT_DATA).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct KeyboardInputData {
    pub unit_id: u16,
    pub make_code: u16,
    pub flags: u16,
    pub reserved: u16,
    pub extra_information: u32,
}

/// Mouse input data (matches kernel MOUSE_INPUT_DATA).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MouseInputData {
    pub unit_id: u16,
    pub flags: u16,
    pub button_flags: u16,
    pub button_data: i16,
    pub raw_buttons: u32,
    pub last_x: i32,
    pub last_y: i32,
    pub extra_information: u32,
}

// ============================================================================
// Constants
// ============================================================================

/// Key flags for keyboard events.
pub mod key_flags {
    pub const KEY_MAKE: u16 = 0x0000;
    pub const BREAK: u16 = 0x0001;
    pub const E0: u16 = 0x0002;
    pub const E1: u16 = 0x0004;
}

/// Mouse button flags.
pub mod mouse_button_flags {
    pub const LEFT_BUTTON_DOWN: u16 = 0x0001;
    pub const LEFT_BUTTON_UP: u16 = 0x0002;
    pub const RIGHT_BUTTON_DOWN: u16 = 0x0004;
    pub const RIGHT_BUTTON_UP: u16 = 0x0008;
    pub const MIDDLE_BUTTON_DOWN: u16 = 0x0010;
    pub const MIDDLE_BUTTON_UP: u16 = 0x0020;
    pub const WHEEL: u16 = 0x0400;
    pub const HWHEEL: u16 = 0x0800;
}

/// Mouse movement flags.
pub mod mouse_move_flags {
    pub const MOVE_RELATIVE: u16 = 0x0000;
    pub const MOVE_ABSOLUTE: u16 = 0x0001;
    pub const VIRTUAL_DESKTOP: u16 = 0x0002;
}

/// PS/2 Set 1 scan codes.
pub mod scan_code {
    // Number row
    pub const ESC: u16 = 0x01;
    pub const NUM1: u16 = 0x02;
    pub const NUM2: u16 = 0x03;
    pub const NUM3: u16 = 0x04;
    pub const NUM4: u16 = 0x05;
    pub const NUM5: u16 = 0x06;
    pub const NUM6: u16 = 0x07;
    pub const NUM7: u16 = 0x08;
    pub const NUM8: u16 = 0x09;
    pub const NUM9: u16 = 0x0A;
    pub const NUM0: u16 = 0x0B;
    pub const MINUS: u16 = 0x0C;
    pub const EQUALS: u16 = 0x0D;
    pub const BACKSPACE: u16 = 0x0E;

    // Top row
    pub const TAB: u16 = 0x0F;
    pub const Q: u16 = 0x10;
    pub const W: u16 = 0x11;
    pub const E: u16 = 0x12;
    pub const R: u16 = 0x13;
    pub const T: u16 = 0x14;
    pub const Y: u16 = 0x15;
    pub const U: u16 = 0x16;
    pub const I: u16 = 0x17;
    pub const O: u16 = 0x18;
    pub const P: u16 = 0x19;
    pub const LEFT_BRACKET: u16 = 0x1A;
    pub const RIGHT_BRACKET: u16 = 0x1B;
    pub const ENTER: u16 = 0x1C;

    // Home row
    pub const LEFT_CTRL: u16 = 0x1D;
    pub const A: u16 = 0x1E;
    pub const S: u16 = 0x1F;
    pub const D: u16 = 0x20;
    pub const F: u16 = 0x21;
    pub const G: u16 = 0x22;
    pub const H: u16 = 0x23;
    pub const J: u16 = 0x24;
    pub const K: u16 = 0x25;
    pub const L: u16 = 0x26;
    pub const SEMICOLON: u16 = 0x27;
    pub const APOSTROPHE: u16 = 0x28;
    pub const GRAVE: u16 = 0x29;

    // Bottom row
    pub const LEFT_SHIFT: u16 = 0x2A;
    pub const BACKSLASH: u16 = 0x2B;
    pub const Z: u16 = 0x2C;
    pub const X: u16 = 0x2D;
    pub const C: u16 = 0x2E;
    pub const V: u16 = 0x2F;
    pub const B: u16 = 0x30;
    pub const N: u16 = 0x31;
    pub const M: u16 = 0x32;
    pub const COMMA: u16 = 0x33;
    pub const PERIOD: u16 = 0x34;
    pub const SLASH: u16 = 0x35;
    pub const RIGHT_SHIFT: u16 = 0x36;

    // Misc
    pub const NUMPAD_MULTIPLY: u16 = 0x37;
    pub const LEFT_ALT: u16 = 0x38;
    pub const SPACE: u16 = 0x39;
    pub const CAPS_LOCK: u16 = 0x3A;

    // Function keys
    pub const F1: u16 = 0x3B;
    pub const F2: u16 = 0x3C;
    pub const F3: u16 = 0x3D;
    pub const F4: u16 = 0x3E;
    pub const F5: u16 = 0x3F;
    pub const F6: u16 = 0x40;
    pub const F7: u16 = 0x41;
    pub const F8: u16 = 0x42;
    pub const F9: u16 = 0x43;
    pub const F10: u16 = 0x44;
    pub const NUM_LOCK: u16 = 0x45;
    pub const SCROLL_LOCK: u16 = 0x46;

    // Numpad
    pub const NUMPAD7: u16 = 0x47;
    pub const NUMPAD8: u16 = 0x48;
    pub const NUMPAD9: u16 = 0x49;
    pub const NUMPAD_MINUS: u16 = 0x4A;
    pub const NUMPAD4: u16 = 0x4B;
    pub const NUMPAD5: u16 = 0x4C;
    pub const NUMPAD6: u16 = 0x4D;
    pub const NUMPAD_PLUS: u16 = 0x4E;
    pub const NUMPAD1: u16 = 0x4F;
    pub const NUMPAD2: u16 = 0x50;
    pub const NUMPAD3: u16 = 0x51;
    pub const NUMPAD0: u16 = 0x52;
    pub const NUMPAD_PERIOD: u16 = 0x53;

    pub const F11: u16 = 0x57;
    pub const F12: u16 = 0x58;

    // Extended keys (use with key_flags::E0)
    // These share base scan codes with numpad keys.
    // Navigation: E0+NUMPAD7=Home, E0+NUMPAD8=Up, E0+NUMPAD9=PageUp, etc.
    // Modifiers: E0+LEFT_CTRL=RightCtrl, E0+LEFT_ALT=RightAlt
    // E0+ENTER=NumpadEnter, E0+SLASH=NumpadDivide
    pub const LEFT_WIN: u16 = 0x5B; // E0 prefix required
}

// ============================================================================
// Device handle
// ============================================================================

#[cfg(target_os = "windows")]
use windows::core::PCSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{
    CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE,
};
#[cfg(target_os = "windows")]
use windows::Win32::Storage::FileSystem::{
    CreateFileA, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::DeviceIoControl;

// ---- Shared device open helper ----

#[cfg(target_os = "windows")]
fn open_device(device_path: &str) -> Option<HANDLE> {
    let handle = unsafe {
        CreateFileA(
            PCSTR(device_path.as_ptr()),
            GENERIC_READ.0 | GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            None,
        )
    };

    match handle {
        Ok(h) if h != INVALID_HANDLE_VALUE => Some(h),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn send_ioctl(handle: HANDLE, ioctl: u32, data: &[u8]) -> windows::core::Result<u32> {
    let mut bytes_returned = 0u32;
    unsafe {
        DeviceIoControl(
            handle,
            ioctl,
            Some(data.as_ptr() as *const c_void),
            data.len() as u32,
            None,
            0,
            Some(&mut bytes_returned),
            None,
        )?;
    }
    Ok(bytes_returned)
}

// ============================================================================
// Keyboard
// ============================================================================

/// Handle to the AllunoHInpuD keyboard control device.
#[cfg(target_os = "windows")]
pub struct AllunoHinpudKeyboard {
    handle: HANDLE,
}

#[cfg(target_os = "windows")]
impl AllunoHinpudKeyboard {
    /// Open the keyboard control device.
    pub fn new() -> Option<Self> {
        open_device("\\\\.\\KeyboardAllunoHInpuD\0").map(|h| Self { handle: h })
    }

    /// Deprecated alias for [`new()`](Self::new).
    #[deprecated(note = "use new() instead")]
    pub fn open() -> Option<Self> {
        Self::new()
    }

    /// Send raw input data to the driver. Returns bytes consumed.
    pub fn send_raw(&self, data: &[u8]) -> windows::core::Result<u32> {
        send_ioctl(self.handle, IOCTL_KEYBOARD_SEND, data)
    }

    /// Send a raw keyboard event (single press or release).
    pub fn send_key_raw(&self, scan_code: u16, flags: u16) -> windows::core::Result<u32> {
        let data = KeyboardInputData {
            unit_id: 0,
            make_code: scan_code,
            flags,
            reserved: 0,
            extra_information: 0,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &data as *const KeyboardInputData as *const u8,
                size_of::<KeyboardInputData>(),
            )
        };
        self.send_raw(bytes)
    }

    /// Press a key (key down).
    pub fn press_key(&self, scan_code: u16) -> windows::core::Result<u32> {
        self.send_key_raw(scan_code, key_flags::KEY_MAKE)
    }

    /// Release a key (key up).
    pub fn release_key(&self, scan_code: u16) -> windows::core::Result<u32> {
        self.send_key_raw(scan_code, key_flags::BREAK)
    }

    /// Send a key (press + release).
    pub fn send_key(&self, scan_code: u16) -> windows::core::Result<()> {
        self.press_key(scan_code)?;
        self.release_key(scan_code)?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
impl Drop for AllunoHinpudKeyboard {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub struct AllunoHinpudKeyboard;

#[cfg(not(target_os = "windows"))]
impl AllunoHinpudKeyboard {
    pub fn new() -> Option<Self> {
        None
    }

    #[deprecated(note = "use new() instead")]
    pub fn open() -> Option<Self> {
        Self::new()
    }
    pub fn send_key(&self, _: u16) -> Result<(), String> {
        Ok(())
    }
    pub fn press_key(&self, _: u16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn release_key(&self, _: u16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_key_raw(&self, _: u16, _: u16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_raw(&self, _: &[u8]) -> Result<u32, String> {
        Ok(0)
    }
}

// ============================================================================
// Mouse
// ============================================================================

/// Handle to the AllunoHInpuD mouse control device.
#[cfg(target_os = "windows")]
pub struct AllunoHinpudMouse {
    handle: HANDLE,
}

#[cfg(target_os = "windows")]
impl AllunoHinpudMouse {
    /// Open the mouse control device.
    pub fn new() -> Option<Self> {
        open_device("\\\\.\\MouseAllunoHInpuD\0").map(|h| Self { handle: h })
    }

    /// Deprecated alias for [`new()`](Self::new).
    #[deprecated(note = "use new() instead")]
    pub fn open() -> Option<Self> {
        Self::new()
    }

    /// Send raw input data to the driver. Returns bytes consumed.
    pub fn send_raw(&self, data: &[u8]) -> windows::core::Result<u32> {
        send_ioctl(self.handle, IOCTL_MOUSE_SEND, data)
    }

    /// Send a mouse move event.
    /// - `absolute = false`: relative move (dx, dy from current position)
    /// - `absolute = true`: absolute move (x, y in 0-65535 virtual desktop coordinates)
    pub fn send_move(&self, x: i32, y: i32, absolute: bool) -> windows::core::Result<u32> {
        let flags = if absolute {
            mouse_move_flags::MOVE_ABSOLUTE | mouse_move_flags::VIRTUAL_DESKTOP
        } else {
            mouse_move_flags::MOVE_RELATIVE
        };
        self.send_move_raw(x, y, flags)
    }

    fn send_move_raw(&self, x: i32, y: i32, flags: u16) -> windows::core::Result<u32> {
        let data = MouseInputData {
            unit_id: 0,
            flags,
            button_flags: 0,
            button_data: 0,
            raw_buttons: 0,
            last_x: x,
            last_y: y,
            extra_information: 0,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &data as *const MouseInputData as *const u8,
                size_of::<MouseInputData>(),
            )
        };
        self.send_raw(bytes)
    }

    /// Send a mouse button event.
    pub fn send_button(&self, button_flags: u16) -> windows::core::Result<u32> {
        let data = MouseInputData {
            unit_id: 0,
            flags: 0,
            button_flags,
            button_data: 0,
            raw_buttons: 0,
            last_x: 0,
            last_y: 0,
            extra_information: 0,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &data as *const MouseInputData as *const u8,
                size_of::<MouseInputData>(),
            )
        };
        self.send_raw(bytes)
    }

    /// Send a vertical mouse wheel event.
    /// Positive = scroll up, negative = scroll down.
    /// One notch = 120 units (WHEEL_DELTA).
    pub fn send_wheel(&self, delta: i16) -> windows::core::Result<u32> {
        let data = MouseInputData {
            unit_id: 0,
            flags: 0,
            button_flags: mouse_button_flags::WHEEL,
            button_data: delta,
            raw_buttons: 0,
            last_x: 0,
            last_y: 0,
            extra_information: 0,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &data as *const MouseInputData as *const u8,
                size_of::<MouseInputData>(),
            )
        };
        self.send_raw(bytes)
    }

    /// Send a horizontal mouse wheel event.
    /// Positive = scroll right, negative = scroll left.
    /// One notch = 120 units (WHEEL_DELTA).
    pub fn send_hwheel(&self, delta: i16) -> windows::core::Result<u32> {
        let data = MouseInputData {
            unit_id: 0,
            flags: 0,
            button_flags: mouse_button_flags::HWHEEL,
            button_data: delta,
            raw_buttons: 0,
            last_x: 0,
            last_y: 0,
            extra_information: 0,
        };
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &data as *const MouseInputData as *const u8,
                size_of::<MouseInputData>(),
            )
        };
        self.send_raw(bytes)
    }
}

#[cfg(target_os = "windows")]
impl Drop for AllunoHinpudMouse {
    fn drop(&mut self) {
        if !self.handle.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub struct AllunoHinpudMouse;

#[cfg(not(target_os = "windows"))]
impl AllunoHinpudMouse {
    pub fn new() -> Option<Self> {
        None
    }

    #[deprecated(note = "use new() instead")]
    pub fn open() -> Option<Self> {
        Self::new()
    }
    pub fn send_move(&self, _: i32, _: i32, _: bool) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_button(&self, _: u16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_wheel(&self, _: i16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_hwheel(&self, _: i16) -> Result<u32, String> {
        Ok(0)
    }
    pub fn send_raw(&self, _: &[u8]) -> Result<u32, String> {
        Ok(0)
    }
}
