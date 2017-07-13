#![windows_subsystem = "windows"]

extern crate winapi;
extern crate user32;
extern crate kernel32;

use winapi::{UINT, WPARAM, LPARAM, LRESULT, LPCWSTR};
use winapi::windef::HWND;
use winapi::{HMENU, HICON, HCURSOR, HBRUSH};
use winapi::{HINSTANCE, WNDCLASSW, CS_VREDRAW, CS_HREDRAW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use winapi::WM_DESTROY;

use user32::{RegisterClassW, CreateWindowExW, MessageBoxA, GetDesktopWindow};
use user32::{GetMessageW, TranslateMessage, DispatchMessageW};
use user32::{DefWindowProcW, PostQuitMessage};

use std::mem::zeroed;
use std::ffi::OsStr;
use std::ptr::null;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

fn main() {
    init_window(480, 360)
}

fn init_window(width: i32, height: i32) {

    let wnd = WNDCLASSW {
        style: CS_VREDRAW | CS_HREDRAW,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0 as HINSTANCE,
        hIcon: 0 as HICON,
        hCursor: 0 as HCURSOR,
        hbrBackground: 16 as HBRUSH,
        lpszMenuName: 0 as LPCWSTR,
        lpszClassName: to_wstring("Rustering_engine"),
    };

    unsafe {
        RegisterClassW(&wnd);
        let h_wnd_desktop = user32::GetDesktopWindow();
        let window = CreateWindowExW(
            0,
            to_wstring("Rustering_engine") as *mut _,
            to_wstring("Rustering_engine") as *mut _,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0,
            0,
            width,
            height,
            h_wnd_desktop,
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );

        let mut msg = zeroed();
        while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn to_wstring(str: &str) -> *const u16 {
    let v: Vec<u16> = OsStr::new(str).encode_wide().chain(once(0)).collect();
    v.as_ptr()
}

pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(h_wnd, msg, w_param, l_param),
    }

}
