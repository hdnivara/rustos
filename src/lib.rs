#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;

const QEMU_ISA_DEBUG_EXIT_PORT: u16 = 0xf4;

// QEMU's isa-debug-exit exits the device (i/e., OS in our case) and
// sets the exit_code to a value.
//
// The exit code is (value << 1) | 1 where value is read from iobase
// port passed via cmdline args (done in Cargo.toml in our case).
//
// isa-debug-exit uses port-mapped I/O to read data. Port-mapped I/O
// uses special 'in' and 'out' instructions to write and read data from
// the port. We use x86_64 crate which provides high-level APIs for
// in/out instead of inline assembly.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(QEMU_ISA_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

// Entry point for `cargo xtest`.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() {
    test_main();
    loop {}
}

// Panic handler -- called on any panic.
//
// This function should never return, and thus return type is marked as
// '!' meaning returns "never" type.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}
