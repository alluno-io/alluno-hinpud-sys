//! AllunoHInpuD test — verifies hardware-level input emulation.
//!
//! Usage:
//!   alluno-hinpud-test                  Monitor keyboard + mouse events
//!   alluno-hinpud-test --driver         Send via driver (expect HARDWARE)
//!   alluno-hinpud-test --sendinput      Send via SendInput (expect INJECTED)

#[cfg(target_os = "windows")]
mod test_impl {
    use std::mem;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::thread;
    use std::time::Duration;

    use windows::Win32::Foundation::*;
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    use alluno_hinpud_sys::*;

    static KB_HW: AtomicU32 = AtomicU32::new(0);
    static KB_INJ: AtomicU32 = AtomicU32::new(0);
    static MS_HW: AtomicU32 = AtomicU32::new(0);
    static MS_INJ: AtomicU32 = AtomicU32::new(0);
    static COUNTING: AtomicBool = AtomicBool::new(false);

    pub unsafe extern "system" fn keyboard_hook(
        code: i32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if code >= 0 {
            let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let hw = kb.flags.0 & 0x10 == 0; // !LLKHF_INJECTED

            if COUNTING.load(Ordering::Relaxed) {
                if hw {
                    KB_HW.fetch_add(1, Ordering::Relaxed);
                } else {
                    KB_INJ.fetch_add(1, Ordering::Relaxed);
                }
            }

            let action = if wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN {
                "DOWN"
            } else {
                "UP  "
            };

            println!(
                "  key   scan=0x{:04X} {} {}",
                kb.scanCode,
                action,
                if hw { "[HARDWARE]" } else { "[INJECTED]" }
            );
        }
        CallNextHookEx(None, code, wparam, lparam)
    }

    pub unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code >= 0 {
            let ms = &*(lparam.0 as *const MSLLHOOKSTRUCT);
            let hw = ms.flags & 0x01 == 0; // !LLMHF_INJECTED

            if COUNTING.load(Ordering::Relaxed) {
                if hw {
                    MS_HW.fetch_add(1, Ordering::Relaxed);
                } else {
                    MS_INJ.fetch_add(1, Ordering::Relaxed);
                }
            }

            let action = match wparam.0 as u32 {
                0x0200 => "MOVE",
                0x0201 => "L-DOWN",
                0x0202 => "L-UP",
                0x0204 => "R-DOWN",
                0x0205 => "R-UP",
                0x0207 => "M-DOWN",
                0x0208 => "M-UP",
                0x020A => "WHEEL",
                _ => "???",
            };

            // Skip printing MOVE in monitor mode (too spammy)
            if wparam.0 as u32 != 0x0200 || COUNTING.load(Ordering::Relaxed) {
                println!(
                    "  mouse {:8} pos=({:5},{:5}) {}",
                    action,
                    ms.pt.x,
                    ms.pt.y,
                    if hw { "[HARDWARE]" } else { "[INJECTED]" }
                );
            }
        }
        CallNextHookEx(None, code, wparam, lparam)
    }

    fn send_via_driver() {
        println!("\n=== AllunoHInpuD Driver Test ===");

        let kbd = match AllunoHinpudKeyboard::open() {
            Some(k) => k,
            None => {
                println!("ERROR: Could not open keyboard driver.");
                println!("  Is the AllunoHInpuD driver installed?");
                std::process::exit(1);
            }
        };

        let mou = match AllunoHinpudMouse::open() {
            Some(m) => m,
            None => {
                println!("ERROR: Could not open mouse driver.");
                println!("  Is the AllunoHInpuD driver installed?");
                std::process::exit(1);
            }
        };

        println!("Driver opened. Sending input in 2 seconds...");
        println!("DO NOT touch keyboard or mouse during test!\n");
        thread::sleep(Duration::from_secs(2));

        COUNTING.store(true, Ordering::Relaxed);

        // Keyboard: harmless modifier keys
        println!("--- Keyboard ---");
        let keys: &[(u16, &str)] = &[
            (scan_code::RIGHT_SHIFT, "RShift"),
            (scan_code::LEFT_SHIFT, "LShift"),
        ];
        for &(scan, name) in keys {
            print!("  Sending {name}...");
            match kbd.press_key(scan) {
                Ok(n) => print!(" down({n})"),
                Err(e) => print!(" down(ERR:{e})"),
            }
            thread::sleep(Duration::from_millis(30));
            match kbd.release_key(scan) {
                Ok(n) => print!(" up({n})"),
                Err(e) => print!(" up(ERR:{e})"),
            }
            thread::sleep(Duration::from_millis(30));
            println!();
        }

        // Mouse: relative move (+1,+1) then back (-1,-1)
        println!("--- Mouse ---");
        print!("  Sending relative move...");
        match mou.send_move(1, 1, false) {
            Ok(n) => print!(" +1,+1({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match mou.send_move(-1, -1, false) {
            Ok(n) => print!(" -1,-1({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        print!("  Sending left click...");
        match mou.send_button(mouse_button_flags::LEFT_BUTTON_DOWN) {
            Ok(n) => print!(" down({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match mou.send_button(mouse_button_flags::LEFT_BUTTON_UP) {
            Ok(n) => print!(" up({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        print!("  Sending right click...");
        match mou.send_button(mouse_button_flags::RIGHT_BUTTON_DOWN) {
            Ok(n) => print!(" down({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match mou.send_button(mouse_button_flags::RIGHT_BUTTON_UP) {
            Ok(n) => print!(" up({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        // Mouse: wheel
        print!("  Sending wheel...");
        match mou.send_wheel(120) {
            Ok(n) => print!(" up({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match mou.send_wheel(-120) {
            Ok(n) => print!(" down({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        // Mouse: hwheel
        print!("  Sending hwheel...");
        match mou.send_hwheel(120) {
            Ok(n) => print!(" right({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match mou.send_hwheel(-120) {
            Ok(n) => print!(" left({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        // Keyboard: extended key (Right Ctrl = E0 + LEFT_CTRL)
        println!("--- Extended Keys ---");
        print!("  Sending RCtrl (E0)...");
        match kbd.send_key_raw(scan_code::LEFT_CTRL, key_flags::E0) {
            Ok(n) => print!(" down({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        match kbd.send_key_raw(scan_code::LEFT_CTRL, key_flags::BREAK | key_flags::E0) {
            Ok(n) => print!(" up({n})"),
            Err(e) => print!(" ERR:{e}"),
        }
        thread::sleep(Duration::from_millis(30));
        println!();

        thread::sleep(Duration::from_millis(100));
        COUNTING.store(false, Ordering::Relaxed);
        print_summary("AllunoHInpuD");
    }

    fn send_via_sendinput() {
        println!("\n=== SendInput Baseline Test ===");
        println!("Sending input in 2 seconds...\n");
        thread::sleep(Duration::from_secs(2));

        COUNTING.store(true, Ordering::Relaxed);

        // Keyboard
        println!("--- Keyboard ---");
        let keys: &[(u16, &str)] = &[
            (scan_code::RIGHT_SHIFT, "RShift"),
            (scan_code::LEFT_SHIFT, "LShift"),
        ];
        for &(scan, name) in keys {
            print!("  Sending {name} via SendInput...");
            sendinput_key(scan, false);
            thread::sleep(Duration::from_millis(30));
            sendinput_key(scan, true);
            thread::sleep(Duration::from_millis(30));
            println!(" done");
        }

        // Mouse: relative move
        println!("--- Mouse ---");
        print!("  Sending relative move via SendInput...");
        sendinput_mouse(MOUSEEVENTF_MOVE, 1, 1);
        thread::sleep(Duration::from_millis(30));
        sendinput_mouse(MOUSEEVENTF_MOVE, -1, -1);
        thread::sleep(Duration::from_millis(30));
        println!(" done");

        // Mouse: left click
        print!("  Sending left click via SendInput...");
        sendinput_mouse(MOUSEEVENTF_LEFTDOWN, 0, 0);
        thread::sleep(Duration::from_millis(30));
        sendinput_mouse(MOUSEEVENTF_LEFTUP, 0, 0);
        thread::sleep(Duration::from_millis(30));
        println!(" done");

        thread::sleep(Duration::from_millis(100));
        COUNTING.store(false, Ordering::Relaxed);
        print_summary("SendInput");
    }

    fn sendinput_key(scan: u16, up: bool) {
        let mut flags = KEYEVENTF_SCANCODE;
        if up {
            flags |= KEYEVENTF_KEYUP;
        }
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: scan,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe {
            SendInput(&[input], mem::size_of::<INPUT>() as i32);
        }
    }

    fn sendinput_mouse(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32) {
        let input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx,
                    dy,
                    mouseData: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe {
            SendInput(&[input], mem::size_of::<INPUT>() as i32);
        }
    }

    fn print_summary(method: &str) {
        let kb_hw = KB_HW.load(Ordering::Relaxed);
        let kb_inj = KB_INJ.load(Ordering::Relaxed);
        let ms_hw = MS_HW.load(Ordering::Relaxed);
        let ms_inj = MS_INJ.load(Ordering::Relaxed);
        let total_hw = kb_hw + ms_hw;
        let total_inj = kb_inj + ms_inj;
        let total = total_hw + total_inj;

        println!("\n====================================");
        println!("  Results ({method})");
        println!("====================================");
        println!("  Keyboard: {kb_hw} HARDWARE, {kb_inj} INJECTED");
        println!("  Mouse:    {ms_hw} HARDWARE, {ms_inj} INJECTED");
        println!("  Total:    {total_hw} HARDWARE, {total_inj} INJECTED");

        if method == "SendInput" {
            if total_inj == total && total > 0 {
                println!("\n  PASS: all events [INJECTED] (expected)");
            } else {
                println!("\n  UNEXPECTED: {total_hw} were [HARDWARE]");
            }
        } else if total_hw == total && total > 0 {
            println!("\n  PASS: all events [HARDWARE] — no INJECTED flag!");
        } else if total_inj > 0 {
            println!("\n  FAIL: {total_inj} events had [INJECTED] flag");
        } else {
            println!("\n  NO EVENTS — is the driver installed?");
        }
        println!("====================================\n");
        std::process::exit(0);
    }

    pub fn run(mode: &str) {
        println!("AllunoHInpuD Test");
        println!("=================\n");

        match mode {
            "--driver" => {
                println!("Mode: Driver (expect [HARDWARE])");
                thread::spawn(send_via_driver);
            }
            "--sendinput" => {
                println!("Mode: SendInput baseline (expect [INJECTED])");
                thread::spawn(send_via_sendinput);
            }
            _ => {
                println!("Mode: Monitor (press keys / move mouse to see flags)");
                println!("  [HARDWARE] = real hardware or kernel-level emulation");
                println!("  [INJECTED] = SendInput/keybd_event");
                println!("\nUsage:");
                println!("  alluno-hinpud-test --driver     Test driver (expect HARDWARE)");
                println!("  alluno-hinpud-test --sendinput  Test SendInput (expect INJECTED)");
                println!("\nPress Ctrl+C to exit.\n");
            }
        }

        // Install both hooks and run message loop
        unsafe {
            let kb_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)
                .expect("Failed to install keyboard hook");
            let ms_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), None, 0)
                .expect("Failed to install mouse hook");

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = UnhookWindowsHookEx(kb_hook);
            let _ = UnhookWindowsHookEx(ms_hook);
        }
    }
}

#[cfg(target_os = "windows")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    test_impl::run(mode);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("This tool only works on Windows.");
    std::process::exit(1);
}
