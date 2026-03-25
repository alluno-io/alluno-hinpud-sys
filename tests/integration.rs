//! Integration tests for alluno-hinpud-sys
//!
//! Requires the AllunoHInpuD driver to be installed.
//! Run with: cargo test -- --nocapture

use alluno_hinpud_sys::*;

#[test]
fn test_open_keyboard() {
    let kbd = AllunoHinpud::open_keyboard();
    assert!(
        kbd.is_some(),
        "Failed to open keyboard — is AllunoHInpuD installed?"
    );
}

#[test]
fn test_open_mouse() {
    let mou = AllunoHinpud::open_mouse();
    assert!(
        mou.is_some(),
        "Failed to open mouse — is AllunoHInpuD installed?"
    );
}

#[test]
fn test_send_key() {
    let kbd = AllunoHinpud::open_keyboard().expect("driver not installed");
    let result = kbd.send_key(scan_code::RIGHT_SHIFT);
    assert!(result.is_ok(), "send_key failed");
}

#[test]
fn test_press_release_key() {
    let kbd = AllunoHinpud::open_keyboard().expect("driver not installed");
    let result = kbd.press_key(scan_code::RIGHT_SHIFT);
    assert!(result.is_ok(), "press_key failed");
    let result = kbd.release_key(scan_code::RIGHT_SHIFT);
    assert!(result.is_ok(), "release_key failed");
}

#[test]
fn test_mouse_relative_move() {
    let mou = AllunoHinpud::open_mouse().expect("driver not installed");
    let result = mou.send_move(0, 0, false);
    assert!(result.is_ok(), "send_move relative failed");
}

#[test]
fn test_mouse_absolute_move() {
    let mou = AllunoHinpud::open_mouse().expect("driver not installed");
    let result = mou.send_move(32767, 32767, true);
    assert!(result.is_ok(), "send_move absolute failed");
}

#[test]
fn test_mouse_button() {
    let mou = AllunoHinpud::open_mouse().expect("driver not installed");
    let result = mou.send_button(mouse_button_flags::RIGHT_BUTTON_DOWN);
    assert!(result.is_ok(), "send_button down failed");
    let result = mou.send_button(mouse_button_flags::RIGHT_BUTTON_UP);
    assert!(result.is_ok(), "send_button up failed");
}
