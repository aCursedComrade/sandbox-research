use sandbox_research::Profile;
use std::collections::HashMap;
use windows_sys::Win32::{Foundation::CloseHandle, System::Threading::UnregisterWait};

#[derive(Debug, Clone)]
/// Profile with states associated to it
pub(crate) struct Managed {
    /// The profile data
    pub conf: Profile,
    /// Handle to the process
    pub h_process: isize,
    /// Handle to the main thread of the process
    h_thread: isize,
    /// WMI wait handle
    pub wait_handle: isize,
}

impl Managed {
    pub fn new(conf: Profile, h_process: isize, h_thread: isize) -> Self {
        Self {
            conf,
            h_process,
            h_thread,
            wait_handle: 0,
        }
    }
}

impl Drop for Managed {
    fn drop(&mut self) {
        unsafe {
            if self.wait_handle != 0 {
                UnregisterWait(self.wait_handle);
            }
            CloseHandle(self.h_thread);
            CloseHandle(self.h_process);
        }
    }
}

/// State list holding profile data (server)
pub(crate) type ManagedList = HashMap<u32, Managed>;

pub(crate) trait ListUtils {
    fn add_profile(&mut self, profile: Managed);
}

impl ListUtils for ManagedList {
    fn add_profile(&mut self, profile: Managed) {
        self.insert(profile.conf.id, profile);
    }
}
