mod hooks;
mod types;

use windows_sys::Win32::{
    Foundation::{BOOL, HMODULE},
    System::LibraryLoader::FreeLibraryAndExitThread,
};

unsafe fn init() {
    if hooks::install_hooks().is_ok() {
        println!("[+] Hooked all functions successfully");
    } else {
        println!("[!] Did not hook all functions successfully");
    }
}

#[no_mangle]
extern "system" fn DllMain(dll_main: HMODULE, call_reason: u32, _: *mut ()) -> BOOL {
    unsafe {
        match call_reason {
            1 => {
                init();
            }
            0 => {
                FreeLibraryAndExitThread(dll_main, 0);
            }
            _ => {}
        }
    }

    BOOL::from(true)
}
