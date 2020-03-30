// By default, Rust compiler will link all the programs to the default
// standard library given by the OS. Since we are writing an OS, we
// should tell the compiler to not to link with any standard library.
#![no_std]
// Tell the compiler that we are not interested in the typical runtime
// to start.
#![no_main]

use core::panic::PanicInfo;

// Panic handler -- called on any panic.
//
// This function should never return, and thus return type is marked as
// '!' meaning returns "never" type.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Instruct the Rust compiler to not not mangle the name of this
// function as we actually need a function named "_start()". Without
// this attribute, compiler would garble this function's name to ensure
// function name uniqueness.
#[no_mangle]
// This is the entry point. Linker looks for a function named '_start()'.
//
// 'extern "C"' tells the compiler to use C calling convention instead
// of Rust calling convention.
pub extern "C" fn _start() -> ! {
    loop {}
}
