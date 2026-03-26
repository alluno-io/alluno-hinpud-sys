//! Integration tests for alluno-hinpud-sys
//!
//! Tests skip gracefully if the AllunoHInpuD driver is not installed.
//! Run with: cargo test -- --nocapture

use alluno_hinpud_sys::*;

#[test]
fn test_open_keyboard() {
    let _ = AllunoHinpudKeyboard::open();
}

#[test]
fn test_open_mouse() {
    let _ = AllunoHinpudMouse::open();
}

#[test]
fn test_send_key() {
    let Some(kbd) = AllunoHinpudKeyboard::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(kbd.send_key(scan_code::RIGHT_SHIFT).is_ok());
}

#[test]
fn test_press_release_key() {
    let Some(kbd) = AllunoHinpudKeyboard::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(kbd.press_key(scan_code::RIGHT_SHIFT).is_ok());
    assert!(kbd.release_key(scan_code::RIGHT_SHIFT).is_ok());
}

#[test]
fn test_mouse_relative_move() {
    let Some(mou) = AllunoHinpudMouse::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(mou.send_move(0, 0, false).is_ok());
}

#[test]
fn test_mouse_absolute_move() {
    let Some(mou) = AllunoHinpudMouse::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(mou.send_move(32767, 32767, true).is_ok());
}

#[test]
fn test_mouse_button() {
    let Some(mou) = AllunoHinpudMouse::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(mou
        .send_button(mouse_button_flags::RIGHT_BUTTON_DOWN)
        .is_ok());
    assert!(mou.send_button(mouse_button_flags::RIGHT_BUTTON_UP).is_ok());
}

#[test]
fn test_mouse_wheel() {
    let Some(mou) = AllunoHinpudMouse::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(mou.send_wheel(120).is_ok());
    assert!(mou.send_wheel(-120).is_ok());
}

#[test]
fn test_mouse_hwheel() {
    let Some(mou) = AllunoHinpudMouse::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    assert!(mou.send_hwheel(120).is_ok());
    assert!(mou.send_hwheel(-120).is_ok());
}

#[test]
fn test_extended_key() {
    let Some(kbd) = AllunoHinpudKeyboard::open() else {
        eprintln!("skipped: driver not installed");
        return;
    };
    // Right Ctrl = E0 + LEFT_CTRL
    assert!(kbd
        .send_key_raw(scan_code::LEFT_CTRL, key_flags::E0)
        .is_ok());
    assert!(kbd
        .send_key_raw(scan_code::LEFT_CTRL, key_flags::BREAK | key_flags::E0)
        .is_ok());
}
