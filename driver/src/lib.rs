#![no_std]
#![no_main]

// BUG: WDF specific methods are not properly baked into `windows-rs` yet.
// It has [`wdk_sys::macros::call_unsafe_wdf_function_binding`] macro to call
// WDF methods by giving it the EXACT function name as the first arguement,
// followed by parameters of that method. WDM approach might be easier for this prototype

// TODO: Current plan is to use WDM to create a non-pnp driver

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    wdk::println!("sandboxdrv PANIC: {info}");
    loop {}
}

extern crate alloc;

#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

mod device;
mod driver;
