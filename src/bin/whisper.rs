#![allow(dead_code)]
use std::ffi::c_void;
use windows_sys::s;
use windows_sys::Win32::System::IO::DeviceIoControl;
use windows_sys::Win32::{
    Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE},
    Storage::FileSystem::{CreateFileA, OPEN_EXISTING},
};

// Copied from expanded MSVC macros from driver source
const BOXDRV_IO_ECHO: u32 = ((0x00000022) << 16) | (((0x0001) | (0x0002)) << 14) | ((0x800) << 2);
const BOXDRV_IO_READLIST: u32 = ((0x00000022) << 16) | ((0x0001) << 14) | ((0x801) << 2);
const BOXDRV_IO_WRITELIST: u32 = ((0x00000022) << 16) | ((0x0002) << 14) | ((0x802) << 2);
const DEVICE_NAME: *const u8 = s!("\\\\.\\BoxDrv");

#[repr(C)]
#[derive(Default, Debug)]
struct BoxDrvState {
    /// List of PIDs to watch
    watchlist: [usize; 32],
    reg_flt: u32,
    fs_flt: u32,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    unsafe {
        println!("[*] This binary is used to test the interaction with the driver");

        let h_device = CreateFileA(
            DEVICE_NAME,
            GENERIC_READ | GENERIC_WRITE,
            0,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            0,
        );

        if h_device == -1 {
            println!("[!] Failed to open a handle to device, is the driver running?");
            return;
        }

        if let Some(action) = args.get(1) {
            if action == "test" {
                complete_run(h_device);
            } else if action == "read" {
                read_list(h_device);
            } else if action == "write" {
                if let Some(pid) = args.get(2) {
                    write_list(h_device, pid.parse::<usize>().unwrap());
                } else {
                    println!("[!] Missing target PID");
                }
            } else {
                println!("[!] Nope 2x");
            }
        } else {
            println!("[!] Nope");
        }

        CloseHandle(h_device);
        println!("[*] Done");
    }
}

fn echo(h_device: isize) {
    let message = "Doing some echoing\0";
    let mut out_buf = vec![0u8; 64];
    let mut out_size: u32 = 0;

    println!("[*] Starting Echo IOCTL");
    println!("[*] Payload: {:?}", message);
    println!("[*] Payload size: {}", message.len());

    unsafe {
        let status = DeviceIoControl(
            h_device,
            BOXDRV_IO_ECHO,
            message.as_ptr() as *const c_void,
            message.len() as u32,
            out_buf.as_mut_ptr() as *mut c_void,
            std::mem::size_of_val(&out_buf) as u32,
            &mut out_size,
            std::ptr::null_mut(),
        );

        if status == 0 {
            println!("[!] Echo IOCTL failed");
            return;
        }

        println!("[+] Bytes received: {}", out_size);
        let response = String::from_utf8(out_buf).unwrap();
        if response.contains(message.trim_matches('\0')) {
            println!("[+] Echo IOCTL succeeded!");
        } else {
            println!("[!] Echo failed, buffers do not match");
        }
    }
}

fn read_list(h_device: isize) {
    let mut out_buf = BoxDrvState::default();
    let mut out_size: u32 = 0;

    unsafe {
        println!("[*] Starting ReadList IOCTL");

        let status = DeviceIoControl(
            h_device,
            BOXDRV_IO_READLIST,
            std::ptr::null(),
            0,
            out_buf.watchlist.as_mut_ptr() as *mut c_void,
            std::mem::size_of_val(&out_buf.watchlist) as u32,
            &mut out_size,
            std::ptr::null_mut(),
        );

        if status == 0 {
            println!("[!] ReadList IOCTL failed");
            return;
        }

        println!("[+] Bytes received: {}", out_size);
        println!("[+] Buffer: {:?}", out_buf);
    }
}

fn write_list(h_device: isize, pid: usize) {
    let mut out_size: u32 = 0;

    unsafe {
        println!("[*] Starting WriteList IOCTL");

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
            println!("[!] WriteList IOCTL failed");
            return;
        }

        println!("[+] Bytes written: {}", out_size);
    }
}

fn complete_run(h_device: isize) {
    echo(h_device);
    read_list(h_device);
    write_list(h_device, 9999);
    read_list(h_device);
}
