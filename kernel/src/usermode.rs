// User mode execution support

pub fn switch_to_user_mode(user_rip: u64, user_rsp: u64) -> ! {
    unsafe {
        // iretq frame: SS, RSP, RFLAGS, CS, RIP
        core::arch::asm!(
            "push 0x23",
            "push {rsp}",
            "pushfq",
            "pop rax",
            "or rax, 0x200",
            "push rax",
            "push 0x1B",
            "push {rip}",
            "iretq",
            rsp = in(reg) user_rsp,
            rip = in(reg) user_rip,
            options(noreturn)
        )
    }
}