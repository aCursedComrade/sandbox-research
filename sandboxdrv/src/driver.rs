use wdk::println;
use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS};

#[export_name = "DriverEntry"]
pub unsafe extern "C" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _device_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("sandboxdrv.sys: Hello world!");

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub unsafe extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("sandboxdrv.sys: Buh bye!");
}
