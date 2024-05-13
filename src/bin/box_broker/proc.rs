use crate::state::Managed;
use sandbox_research::{inject::inject, Profile};
use std::{iter, mem::zeroed, ptr};
use windows_sys::Win32::{
    Foundation::GetLastError,
    System::Threading::{
        CreateProcessW, ResumeThread, TerminateProcess, CREATE_NEW_CONSOLE, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOW,
    },
};

pub fn spawn(profile: Profile) -> Result<Managed, ()> {
    let mut conf = profile.clone();
    let mut cmd = conf.command.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    unsafe {
        let si = zeroed::<STARTUPINFOW>();
        let mut pi = zeroed::<PROCESS_INFORMATION>();

        if CreateProcessW(
            ptr::null(),                           // lpapplicationname
            cmd.as_mut_ptr(),                      // lpcmdline
            ptr::null(),                           // lpprocessattributes
            ptr::null(),                           // lpthreadattributes
            0,                                     // binherithandle
            CREATE_NEW_CONSOLE | CREATE_SUSPENDED, // dwcreationflags
            ptr::null(),                           // lpenvironment
            ptr::null(),                           // lpcurrentdirectory
            &si as *const STARTUPINFOW,            // lpstartupinfo
            &mut pi,                               // lpprocessinfo
        ) == 0
        {
            tracing::error!("Failed to spawn new process: {}", GetLastError());
            return Err(());
        }

        // inject the DLL while it is suspended
        #[cfg(debug_assertions)]
        let path = format!(
            "{}\\debug\\hooked.dll",
            std::env::var("CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY").unwrap()
        ); // works only when used with cargo-make
        #[cfg(not(debug_assertions))]
        let path = "utils\\hooked.dll".to_string();
        if let Err(error) = inject(pi.hProcess, &path) {
            tracing::error!("Failed to inject the DLL to the child process: {}", error);
            stop(pi.hProcess);
            return Err(());
        }

        // sends the PID over to driver
        #[cfg(not(debug_assertions))]
        if !sandbox_research::ioctl::write_list(pi.dwProcessId as usize) {
            tracing::error!("Failed to update driver state, terminating child");
            stop(pi.hProcess);
            return Err(());
        }

        // resume the process
        if ResumeThread(pi.hThread) == u32::MAX {
            tracing::error!("Failed to resume the child process: {}", GetLastError());
            stop(pi.hProcess);
            return Err(());
        }

        conf.pid = pi.dwProcessId;
        Ok(Managed::new(conf, pi.hProcess, pi.hThread))
    }
}

pub fn stop(handle: isize) -> bool {
    // https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess
    unsafe { TerminateProcess(handle, 1) != 0 }
}
