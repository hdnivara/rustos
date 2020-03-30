// By default, Rust compiler will link all the programs to the default
// standard library given by the OS. Since we are writing an OS, we
// should tell the compiler to not to link with any standard library.
#![no_std]
// Tell the compiler that we are not interested in the typical runtime
// to start.
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Panic handler -- called on any panic.
//
// This function should never return, and thus return type is marked as
// '!' meaning returns "never" type.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

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
    use core::fmt::Write;
    use vga_buffer::Colour;
    use vga_buffer::ColourCode;

    vga_buffer::WRITER.lock().write_string("Hello, RustOS!\n");

    vga_buffer::WRITER.lock().write_byte(b'I');
    vga_buffer::WRITER.lock().write_byte(b'n');
    vga_buffer::WRITER.lock().write_byte(b'd');
    vga_buffer::WRITER.lock().write_byte(b'i');
    vga_buffer::WRITER.lock().write_byte(b'v');
    vga_buffer::WRITER.lock().write_byte(b'i');
    vga_buffer::WRITER.lock().write_byte(b'd');
    vga_buffer::WRITER.lock().write_byte(b'u');
    vga_buffer::WRITER.lock().write_byte(b'a');
    vga_buffer::WRITER.lock().write_byte(b'l');
    vga_buffer::WRITER.lock().write_byte(b' ');
    vga_buffer::WRITER.lock().write_byte(b'b');
    vga_buffer::WRITER.lock().write_byte(b'y');
    vga_buffer::WRITER.lock().write_byte(b't');
    vga_buffer::WRITER.lock().write_byte(b'e');
    vga_buffer::WRITER.lock().write_byte(b's');
    vga_buffer::WRITER.lock().write_byte(b'\n');

    write!(
        vga_buffer::WRITER.lock(),
        "Formatted using write macro: {} -- integer and {} -- floating point\n",
        9,
        3.3 + 3.0
    )
    .unwrap();

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

    panic!("We can even panic now!");
}
