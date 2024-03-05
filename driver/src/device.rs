use wdk_sys::{NTSTATUS, WDFDEVICE_INIT};

pub(crate) extern "C" fn _device_create(mut _device_init: &mut WDFDEVICE_INIT) -> NTSTATUS {
    todo!()
}
