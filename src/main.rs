#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use winapi::um::winuser::*;

fn main() {
    unsafe {
        RegisterHotKey(null_mut(), 0, MOD_SHIFT as u32, VK_ESCAPE as u32);
        let mut msg: MSG = std::mem::zeroed();

        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            if msg.message == WM_HOTKEY {
                ShowWindow(GetForegroundWindow(), SW_FORCEMINIMIZE);
            }
        }
    }
}
