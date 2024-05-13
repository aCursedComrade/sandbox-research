use std::{ffi::c_void, os::windows::raw::HANDLE};
use windows_sys::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::BOOL,
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::{
            FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE, FINDEX_INFO_LEVELS, FINDEX_SEARCH_OPS,
            FIND_FIRST_EX_FLAGS, GET_FILEEX_INFO_LEVELS, WIN32_FIND_DATAW,
        },
        System::Threading::{PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOW},
    },
};

pub type CreateProcessW = unsafe extern "system" fn(
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const STARTUPINFOW,
    lpprocessinformation: *mut PROCESS_INFORMATION,
) -> BOOL;

pub type CreateFileW = unsafe extern "system" fn(
    PCWSTR,
    u32,
    FILE_SHARE_MODE,
    *const SECURITY_ATTRIBUTES,
    FILE_CREATION_DISPOSITION,
    FILE_FLAGS_AND_ATTRIBUTES,
    HANDLE,
) -> HANDLE;
pub type CreateDirectoryW = unsafe extern "system" fn(PCWSTR, *const SECURITY_ATTRIBUTES) -> BOOL;
pub type DeleteFileW = unsafe extern "system" fn(PCWSTR) -> BOOL;
pub type RemoveDirectoryW = unsafe extern "system" fn(PCWSTR) -> BOOL;
pub type GetFileAttributesW = unsafe extern "system" fn(PCWSTR) -> u32;
pub type GetFileAttributesExW = unsafe extern "system" fn(PCWSTR, GET_FILEEX_INFO_LEVELS, *mut c_void) -> BOOL;
pub type SetFileAttributesW = unsafe extern "system" fn(PCWSTR, FILE_FLAGS_AND_ATTRIBUTES) -> BOOL;
pub type FindFirstFileW = unsafe extern "system" fn(PCWSTR, *mut WIN32_FIND_DATAW) -> HANDLE;
pub type FindFirstFileExW = unsafe extern "system" fn(
    PCWSTR,
    FINDEX_INFO_LEVELS,
    *mut c_void,
    FINDEX_SEARCH_OPS,
    *const c_void,
    FIND_FIRST_EX_FLAGS,
) -> HANDLE;
