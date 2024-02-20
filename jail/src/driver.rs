use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING};

#[export_name = "DriverEntry"]
pub unsafe extern "system" fn driver_entry(_: &mut DRIVER_OBJECT, _: PCUNICODE_STRING) -> NTSTATUS {
    0
}
