use crate::kernel_main;

unsafe extern "C" {
    static _stack_top: u8;
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "lea rsp, [rip + {stack_top}]",
        "jmp {rust_start}",
        stack_top=sym _stack_top,
        rust_start=sym rust_start,
    );
}

extern "sysv64" fn rust_start() -> ! {
    kernel_main();
}
