#![allow(non_snake_case)]
use crate::types;
use minhook::{MinHook, MH_STATUS};
use std::{
    ffi::{c_void, CString},
    iter,
    os::windows::raw::HANDLE,
};
use windows_sys::{
    core::PCWSTR,
    Win32::{
        Foundation::{BOOL, MAX_PATH},
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS,
            FIND_FIRST_EX_FLAGS, GET_FILEEX_INFO_LEVELS, WIN32_FIND_DATAW,
        },
        System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
    },
};

// Retrieves the function address from given module
fn get_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = module.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();
    let symbol = CString::new(symbol).unwrap();

    unsafe {
        let handle = GetModuleHandleW(module.as_ptr());
        GetProcAddress(handle, symbol.as_ptr() as *const u8).map(|addr| addr as usize)
    }
}

// Decode a string from a pointer to a u16 (wide char) buffer
fn decode_u16(buffer: *const u16) -> String {
    // Build a buffer first with the pointer
    let buffer = unsafe { std::slice::from_raw_parts(buffer, MAX_PATH as usize - 1) };

    // Find the index of the first NULL byte
    let null = buffer.iter().position(|c| *c == '\0' as u16).unwrap_or_default();

    // Decode the buffer taken upto the NULL to produce a string
    String::from_utf16_lossy(&buffer[..null])
}

// Replaces (and essentially redirects) given path strings
fn replace_path(path: String) -> String {
    let uservar = std::env::var("USERPROFILE");
    let mut out = path.clone();

    if let Ok(user) = uservar {
        if path.starts_with(&user) && !path.starts_with(&(user.clone() + "\\Documents\\BoxedData")) {
            let mut new = user.clone();
            new.push_str("\\Documents\\BoxedData");
            out = out.replace(&user, &new);
            println!(">> {} -> {}", path, out);
        }
    }

    out
}

pub unsafe fn install_hooks() -> Result<(), MH_STATUS> {
    let og_address = get_symbol_address("kernel32.dll", "CreateFileW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, CreateFileW as _);
    let og_func: types::CreateFileW = std::mem::transmute(ret_addr.unwrap());
    FN_CREATEFILEW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "CreateDirectoryW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, CreateDirectoryW as _);
    let og_func: types::CreateDirectoryW = std::mem::transmute(ret_addr.unwrap());
    FN_CREATEDIRECTORYW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "DeleteFileW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, DeleteFileW as _);
    let og_func: types::DeleteFileW = std::mem::transmute(ret_addr.unwrap());
    FN_DELETEFILEW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "RemoveDirectoryW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, RemoveDirectoryW as _);
    let og_func: types::RemoveDirectoryW = std::mem::transmute(ret_addr.unwrap());
    FN_REMOVEDIRECTORYW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "GetFileAttributesW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, GetFileAttributesW as _);
    let og_func: types::GetFileAttributesW = std::mem::transmute(ret_addr.unwrap());
    FN_GETFILEATTRIBUTESW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "GetFileAttributesExW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, GetFileAttributesExW as _);
    let og_func: types::GetFileAttributesExW = std::mem::transmute(ret_addr.unwrap());
    FN_GETFILEATTRIBUTESEXW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "SetFileAttributesW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, SetFileAttributesW as _);
    let og_func: types::SetFileAttributesW = std::mem::transmute(ret_addr.unwrap());
    FN_SETFILEATTRIBUTESW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "FindFirstFileW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, FindFirstFileW as _);
    let og_func: types::FindFirstFileW = std::mem::transmute(ret_addr.unwrap());
    FN_FINDFIRSTFILEW = Some(og_func);

    let og_address = get_symbol_address("kernel32.dll", "FindFirstFileExW").unwrap();
    let ret_addr = MinHook::create_hook(og_address as _, FindFirstFileExW as _);
    let og_func: types::FindFirstFileExW = std::mem::transmute(ret_addr.unwrap());
    FN_FINDFIRSTFILEEXW = Some(og_func);

    MinHook::enable_all_hooks()
}

static mut FN_CREATEFILEW: Option<types::CreateFileW> = None;
static mut FN_CREATEDIRECTORYW: Option<types::CreateDirectoryW> = None;
static mut FN_DELETEFILEW: Option<types::DeleteFileW> = None;
static mut FN_REMOVEDIRECTORYW: Option<types::RemoveDirectoryW> = None;
static mut FN_GETFILEATTRIBUTESW: Option<types::GetFileAttributesW> = None;
static mut FN_GETFILEATTRIBUTESEXW: Option<types::GetFileAttributesExW> = None;
static mut FN_SETFILEATTRIBUTESW: Option<types::SetFileAttributesW> = None;
static mut FN_FINDFIRSTFILEW: Option<types::FindFirstFileW> = None;
static mut FN_FINDFIRSTFILEEXW: Option<types::FindFirstFileExW> = None;

unsafe extern "system" fn CreateFileW(
    name: PCWSTR,
    access: u32,
    sharemode: FILE_SHARE_MODE,
    attrs: *const SECURITY_ATTRIBUTES,
    disposition: FILE_CREATION_DISPOSITION,
    flags: FILE_FLAGS_AND_ATTRIBUTES,
    template: HANDLE,
) -> HANDLE {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_CREATEFILEW.unwrap();
    og_func(name.as_ptr(), access, sharemode, attrs, disposition, flags, template)
}

unsafe extern "system" fn CreateDirectoryW(name: PCWSTR, attrs: *const SECURITY_ATTRIBUTES) -> BOOL {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_CREATEDIRECTORYW.unwrap();
    og_func(name.as_ptr(), attrs)
}

unsafe extern "system" fn DeleteFileW(name: PCWSTR) -> BOOL {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_DELETEFILEW.unwrap();
    og_func(name.as_ptr())
}

unsafe extern "system" fn RemoveDirectoryW(name: PCWSTR) -> BOOL {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_REMOVEDIRECTORYW.unwrap();
    og_func(name.as_ptr())
}

unsafe extern "system" fn GetFileAttributesW(name: PCWSTR) -> u32 {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_GETFILEATTRIBUTESW.unwrap();
    og_func(name.as_ptr())
}

unsafe extern "system" fn GetFileAttributesExW(name: PCWSTR, info: GET_FILEEX_INFO_LEVELS, buffer: *mut c_void) -> BOOL {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_GETFILEATTRIBUTESEXW.unwrap();
    og_func(name.as_ptr(), info, buffer)
}

unsafe extern "system" fn SetFileAttributesW(name: PCWSTR, flags: FILE_FLAGS_AND_ATTRIBUTES) -> BOOL {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_SETFILEATTRIBUTESW.unwrap();
    og_func(name.as_ptr(), flags)
}

unsafe extern "system" fn FindFirstFileW(name: PCWSTR, data: *mut WIN32_FIND_DATAW) -> HANDLE {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_FINDFIRSTFILEW.unwrap();
    og_func(name.as_ptr(), data)
}

unsafe extern "system" fn FindFirstFileExW(
    name: PCWSTR,
    infolevel: FINDEX_INFO_LEVELS,
    filedata: *mut c_void,
    searchop: FINDEX_SEARCH_OPS,
    filter: *const c_void,
    flags: FIND_FIRST_EX_FLAGS,
) -> HANDLE {
    let path = replace_path(decode_u16(name));
    let name = path.encode_utf16().chain(iter::once(0)).collect::<Vec<u16>>();

    let og_func = FN_FINDFIRSTFILEEXW.unwrap();
    og_func(name.as_ptr(), infolevel, filedata, searchop, filter, flags)
}
