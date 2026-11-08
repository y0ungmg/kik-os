
// Context switching for x86_64

use core::arch::asm;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rip: u64,
}

pub fn switch_context(old: &mut Context, new: &Context) {
    unsafe {
        asm!(
            "push rbp",
            "mov rbp, rsp",
            "push rbx",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "mov [rdi], r15",
            "mov [rdi+8], r14",
            "mov [rdi+16], r13",
            "mov [rdi+24], r12",
            "mov [rdi+32], rbp",
            "mov [rdi+40], rbx",
            "lea rax, [rip+2f]",
            "mov [rdi+48], rax",
            "mov r15, [rsi]",
            "mov r14, [rsi+8]",
            "mov r13, [rsi+16]",
            "mov r12, [rsi+24]",
            "mov rbp, [rsi+32]",
            "mov rbx, [rsi+40]",
            "mov rax, [rsi+48]",
            "jmp rax",
            "2:",
            in("rdi") old as *mut Context,
            in("rsi") new as *const Context,
            out("rax") _,
        );
    }
}