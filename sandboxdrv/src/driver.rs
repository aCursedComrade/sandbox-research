use wdk::println;
use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, STATUS_SUCCESS};

// TODO: refer to documentation for defining correct details and capabilities
// in INX file. Some work left to be done make a proper DriverEntry and DriverExit routines.

#[export_name = "DriverEntry"]
unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _device_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("sandboxdrv.sys: Hello world!");

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

unsafe extern "C" fn driver_exit(_driver: *mut DRIVER_OBJECT) {
    println!("sandboxdrv.sys: Buh bye!");
}
