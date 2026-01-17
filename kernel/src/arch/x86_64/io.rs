pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;

    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") val,
            in("dx") port,
            options(preserves_flags, nomem, nostack)
        );
    }

    val
}

pub unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("al") val,
            in("dx") port,
            options(preserves_flags, nomem, nostack)
        );
    }
}
