use lazy_static::lazy_static;
use x86_64::structures::gdt::{
    Descriptor, GlobalDescriptorTable, SegmentSelector,
};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4 * 1024;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // unsafe is requires as the data is a mutable static, and
            // thus compiler cannot ensure data-race-free access.
            let start = VirtAddr::from_ptr(unsafe {&STACK});
            let end = start + STACK_SIZE;
            end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();

    // unsafe is required as compiler cannot guarantee code_selector and
    // tss_selector are valid and if invalid, CPU could execute
    // arbitarary memory.
    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
