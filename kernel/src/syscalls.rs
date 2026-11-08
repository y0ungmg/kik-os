// System call interface

pub const SYS_WRITE: u64 = 1;
pub const SYS_EXIT: u64 = 60;

pub fn handle_syscall(num: u64, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    match num {
        SYS_WRITE => {
            // arg1 = fd, arg2 = buffer ptr, arg3 = length
            // For now, just print to serial
            0
        }
        SYS_EXIT => {
            // arg1 = exit code
            0
        }
        _ => {
            // Unknown syscall
            u64::MAX
        }
    }
}