// By default, Rust compiler will link all the programs to the default
// standard library given by the OS. Since we are writing an OS, we
// should tell the compiler to not to link with any standard library.
#![no_std]
// Tell the compiler that we are not interested in the typical runtime
// to start.
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
// Rename the test framework entry-point function to "test_main()".
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

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
    use vga_buffer::Colour;
    use vga_buffer::ColourCode;

    vga_buffer::WRITER.lock().write_string("Hello, RustOS!\n");

    rustos::init();

    // Create a new Writer to write in Red colour.
    let mut w =
        vga_buffer::Writer::new(ColourCode::new(Colour::Red, Colour::Black));
    w.write_byte(b'\n');
    w.write_string("Now writing in Red!\n");

    // Go back original WRITER. Patch the column_pos accordingly.
    vga_buffer::WRITER
        .lock()
        .set_column_pos(w.get_column_pos())
        .unwrap();
    vga_buffer::WRITER
        .lock()
        .write_string("Back to Green again!!\n");

    println!("Printed using {}!\n", "1 real print macro");

    // Run any tests if we are invoked via "cargo xtests".
    #[cfg(test)]
    test_main();

    loop {}
}

// Panic handler -- called on any panic.
//
// This function should never return, and thus return type is marked as
// '!' meaning returns "never" type.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
