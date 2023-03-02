// This function hides the Windows console window. It is used in the
// `init` function of the `winit` crate to hide the console window
// when the `WindowBuilderExtWindows::with_console` method is used.
//
// The `hide_console_window` function is used by the `winit` crate when the
// `WindowBuilderExtWindows::with_console` method is used.
//
// The function gets the handle to the console window and then calls the
// `ShowWindow` function to hide the window.

pub fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}