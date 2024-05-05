use crate::state::Managed;
use sandbox_research::Profile;
use std::{mem::zeroed, ptr};
use windows_sys::Win32::System::Threading::{CreateProcessA, TerminateProcess, PROCESS_INFORMATION, STARTUPINFOA};

pub fn spawn(profile: Profile) -> Result<Managed, ()> {
    let mut conf = profile.clone();
    let mut cmd = conf.command.trim().to_owned() + "\0";

    unsafe {
        let si = zeroed::<STARTUPINFOA>();
        let mut pi = zeroed::<PROCESS_INFORMATION>();

        if CreateProcessA(
            ptr::null(),      // lpapplicationname
            cmd.as_mut_ptr(), // lpcmdline
            ptr::null(),      // lpprocessattributes
            ptr::null(),      // lpthreadattributes
            0,                // binherithandle
            0,                // dwcreationflags
            ptr::null(),      // lpenvironment
            ptr::null(),      // lpcurrentdirectory
            &si,              // lpstartupinfo
            &mut pi,          // lpprocessinfo
        ) == 0
        {
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
