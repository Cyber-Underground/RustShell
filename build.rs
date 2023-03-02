extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
    windows::build!(
        Windows::Win32::System::SystemServices::{PWSTR},
        Windows::Win32::UI::Shell::{ShellExecuteW, SEE_MASK_NOCLOSEPROCESS},
        Windows::Win32::UI::WindowsAndMessaging::{HWND},
    );
}
