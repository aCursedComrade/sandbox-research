mod hooks;
mod types;

use windows_sys::Win32::{
    Foundation::{BOOL, HMODULE},
    System::LibraryLoader::FreeLibraryAndExitThread,
};

pub static mut M_HANDLE: isize = 0;

unsafe fn init() {
    if hooks::install_hooks().is_ok() {
        tracing::info!("[+] Hooked all functions successfully");
    } else {
        tracing::error!("[!] Did not hook all functions successfully");
    }
}

#[no_mangle]
extern "system" fn DllMain(dll_main: HMODULE, call_reason: u32, _: *mut ()) -> BOOL {
    unsafe {
        match call_reason {
            1 => {
                M_HANDLE = dll_main;
                tracing_subscriber::fmt::init();
                std::thread::spawn(|| {
                    init();
                });
            }
            0 => {
                FreeLibraryAndExitThread(dll_main, 0);
            }
            _ => {}
        }
    }

    BOOL::from(true)
}
