// Don't link the Rust standard library
#![no_std]
// Disable all Rust level entry points.
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
