#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use rustos::{exit_qemu, serial_print, serial_println, QemuExitCode};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rustos::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow: stack_overflow... ");

    rustos::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
// Recursively calling a function infinitely would result in stack
// overflow. It in turn leads to dreadful triple fault which SW cannot
// catch and results in processor reset.
fn stack_overflow() {
    stack_overflow();
}

// Custom double-fault handler to not panic and return success for test.
extern "x86-interrupt" fn test_double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    serial_println!("[ok]");
    serial_println!("Test exception: Double fault!");
    serial_println!("{:#?}", stack_frame);
    serial_println!("error_code: {}", error_code);

    exit_qemu(QemuExitCode::Success);

    loop {}
}
