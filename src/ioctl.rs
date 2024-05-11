use std::ffi::{c_void, CString};
use windows_sys::s;
use windows_sys::Win32::System::IO::DeviceIoControl;
use windows_sys::Win32::{
    Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE},
    Storage::FileSystem::{CreateFileA, OPEN_EXISTING},
};

// Copied from expanded macros from the driver source files
const BOXDRV_IO_ECHO: u32 = ((0x00000022) << 16) | (((0x0001) | (0x0002)) << 14) | ((0x800) << 2);
const BOXDRV_IO_READLIST: u32 = ((0x00000022) << 16) | ((0x0001) << 14) | ((0x801) << 2);
const BOXDRV_IO_WRITELIST: u32 = ((0x00000022) << 16) | ((0x0002) << 14) | ((0x802) << 2);
const DEVICE_NAME: *const u8 = s!("\\\\.\\BoxDrv");

fn open_device() -> isize {
    unsafe {
        CreateFileA(
            DEVICE_NAME,
            GENERIC_READ | GENERIC_WRITE,
            0,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            0,
        )
    }
}

/// Echo IOCTL (only for testing).
/// Buffer is hardcoded to 64 bytes
pub fn echo(message: &str) -> bool {
    if message.len() > 64 {
        tracing::error!("Message overflows hardcoded buffer");
        return false;
    }

    let buffer = unsafe { CString::from_vec_unchecked(message.as_bytes().to_vec()) };
    let mut out_buf = vec![0u8; 64];
    let mut out_size: u32 = 0;

    unsafe {
        let h_device = open_device();

        let status = DeviceIoControl(
            h_device,
            BOXDRV_IO_ECHO,
            buffer.as_ptr() as *const c_void,
            std::mem::size_of_val(buffer.as_bytes()) as u32,
            out_buf.as_mut_ptr() as *mut c_void,
            std::mem::size_of_val(&out_buf) as u32,
            &mut out_size,
            std::ptr::null_mut(),
        );

        if status == 0 {
            tracing::error!("Echo IOCTL failed");
            return false;
        }

        CloseHandle(h_device);

        let response = String::from_utf8(out_buf).unwrap();
        if response.eq(&message) {
            tracing::info!("Echo IOCTL succeeded!");
            true
        } else {
            tracing::error!("Echo failed, buffers do not match");
            false
        }
    }
}

/// Reads the watch list from the driver
pub fn read_list() -> Option<Vec<usize>> {
    let mut out_buf = vec![0usize; 32];
    let mut out_size: u32 = 0;

    unsafe {
        let h_device = open_device();

        let status = DeviceIoControl(
            h_device,
            BOXDRV_IO_READLIST,
            std::ptr::null(),
            0,
            out_buf.as_mut_ptr() as *mut c_void,
            std::mem::size_of_val(out_buf.as_slice()) as u32,
            &mut out_size,
            std::ptr::null_mut(),
        );

        if status == 0 {
            tracing::error!("ReadList IOCTL failed");
            return None;
        }

        CloseHandle(h_device);
    };

    Some(out_buf)
}

/// Sends a PID to the watch list of the driver
pub fn write_list(pid: usize) -> bool {
    let mut out_size: u32 = 0;

    unsafe {
        let h_device = open_device();

        let status = DeviceIoControl(
            h_device,
            BOXDRV_IO_WRITELIST,
            &pid as *const _ as *const c_void,
            std::mem::size_of::<usize>() as u32,
            std::ptr::null_mut(),
            0,
            &mut out_size,
            std::ptr::null_mut(),
        );

        if status == 0 {
            tracing::error!("WriteList IOCTL failed");
            return false;
        }

        CloseHandle(h_device);
    };

    true
}
