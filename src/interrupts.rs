use crate::gdt;
use crate::println;
#[cfg(test)]
use crate::{serial_print, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // unsafe is required as compiler cannot gurantee the stack
        // index used here is not already used for another exception.
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame,
) {
    println!("Exception: Breakpoint!\n");
    println!("{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "\nException: Double fault!\n{:#?}\nerror_code: {}\n",
        stack_frame, error_code
    );
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("interrupts: test_breakpoint_exception... ");

    // Raise breakpoint (int3) interrupt.
    x86_64::instructions::interrupts::int3();

    serial_println!("[ok]");
}
