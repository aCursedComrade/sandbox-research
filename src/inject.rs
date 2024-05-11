use rev_toolkit::RTStatus;
use std::{ffi::CString, mem::size_of};
use windows_sys::{
    s, w,
    Win32::{
        Foundation::{CloseHandle, MAX_PATH},
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::{GetModuleHandleW, GetProcAddress},
            Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
            Threading::{CreateRemoteThread, GetExitCodeThread, WaitForSingleObject},
        },
    },
};

// Referenced implementation from my library - https://github.com/aCursedComrade/rev-toolkit/blob/main/src/bin/dll-inject/inject.rs
// TODO maybe try integrating both in the future

pub fn inject(handle: isize, dll_path: &str) -> Result<(), RTStatus> {
    let path = unsafe { CString::from_vec_unchecked(dll_path.as_bytes().to_vec()) };
    let proc_address = unsafe { GetProcAddress(GetModuleHandleW(w!("Kernel32")), s!("LoadLibraryA")) };

    let buffer = unsafe {
        VirtualAllocEx(
            handle,
            std::ptr::null(),
            (MAX_PATH as usize) * size_of::<u16>(),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        )
    };

    if buffer.is_null() {
        return Err(RTStatus::MemoryAllocError);
    }

    let _ = unsafe {
        WriteProcessMemory(
            handle,
            buffer,
            path.as_ptr() as *const std::ffi::c_void,
            (MAX_PATH as usize) * size_of::<u16>(),
            std::ptr::null_mut(),
        )
    };

    let thread = unsafe {
        CreateRemoteThread(
            handle,
            std::ptr::null(),
            0,
            std::mem::transmute(proc_address),
            buffer as *const std::ffi::c_void,
            0,
            std::ptr::null_mut(),
        )
    };

    unsafe {
        WaitForSingleObject(thread, u32::MAX);
        let mut exit_code = 0u32;
        GetExitCodeThread(thread, &mut exit_code);
        CloseHandle(thread);
        VirtualFreeEx(handle, buffer, 0, MEM_RELEASE);
    }

    Ok(())
}
