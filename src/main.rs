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

static HELLO: &[u8] = b"Hello, world!";

// Instruct the Rust compiler to not not mangle the name of this
// function as we actually need a function named "_start()". Without
// this attribute, compiler would garble this function's name to ensure
// function name uniqueness.
#[no_mangle]
// This is the entry point. Linker looks for a function named
// '_start()'.
//
// 'extern "C"' tells the compiler to use C calling convention instead
// of Rust calling convention.
pub extern "C" fn _start() -> ! {
    // VGA text buffer starting address is 0xb8000.
    let vga_buf = 0xb8000 as *mut u8;

    // Set the VGA text buffer to data pointed by HELLO.
    //
    // Each character is represented by two bytes:
    //  - byte0: character code point
    //  - byte1: control byte with bitfields for colour, blinking, etc.
    //
    // Set the colour to 0xb (Cyan).
    //
    // See https://en.wikipedia.org/wiki/VGA-compatible_text_mode#Text_buffer
    for (i, &byte) in HELLO.iter().enumerate() {
        // We are directly writing to VGA text buffer address. Rust
        // compiler has no way of guaranteeing safety, and thus 'unsafe'
        // is required.
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
