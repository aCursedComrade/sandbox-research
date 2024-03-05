use wdk::println;
use wdk_sys::{DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING};

#[no_mangle]
#[export_name = "DriverEntry"]
unsafe extern "system" fn driver_entry(
    mut driver: DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("sandboxdrv: Hello from DriverEntry!");

    driver.DriverUnload = Some(driver_unload);

    0
}

extern "C" fn driver_unload(_driver: *mut DRIVER_OBJECT) {
    println!("sandboxdrv: EvtDriverUnload invoked!");
    println!("sandboxdrv: Buh bye!");
}
