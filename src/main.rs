// Don't link the Rust standard library
#![no_std]
// Disable all Rust level entry points.
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Don't mangle the name of the function.
//
// This function is the entry point, since the linker looks for a function
// called `_start` by default.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
