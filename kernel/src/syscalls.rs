// System call interface

pub const SYS_WRITE: u64 = 1;
pub const SYS_READ: u64 = 0;
pub const SYS_EXIT: u64 = 60;
pub const SYS_GETPID: u64 = 39;
pub const SYS_SLEEP: u64 = 35;

static mut CURRENT_PID: u64 = 1;

pub fn handle_syscall(num: u64, arg1: u64, _arg2: u64, _arg3: u64) -> u64 {
    match num {
        SYS_WRITE => {
            // arg1 = fd, arg2 = buffer ptr, arg3 = length
            // For now, just return success
            0
        }
        SYS_READ => {
            // arg1 = fd, arg2 = buffer ptr, arg3 = length
            0
        }
        SYS_EXIT => {
            // arg1 = exit code
            0
        }
        SYS_GETPID => {
            unsafe { CURRENT_PID }
        }
        SYS_SLEEP => {
            // arg1 = seconds
            0
        }
        _ => {
            // Unknown syscall
            u64::MAX
        }
    }
}
