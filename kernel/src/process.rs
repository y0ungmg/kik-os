// Process management

pub const MAX_PROCESSES: usize = 4;

#[derive(Clone, Copy)]
pub struct Process {
    pub pid: u64,
    pub state: ProcessState,
    pub entry_point: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

impl Process {
    pub fn new(pid: u64, entry_point: u64) -> Self {
        Self {
            pid,
            state: ProcessState::Ready,
            entry_point,
        }
    }
}

static mut PROCESSES: [Option<Process>; MAX_PROCESSES] = [None; MAX_PROCESSES];
static mut PROCESS_COUNT: usize = 0;
static mut NEXT_PID: u64 = 1;

pub fn create_process(entry_point: u64) -> u64 {
    unsafe {
        if PROCESS_COUNT >= MAX_PROCESSES {
            return 0;
        }
        let pid = NEXT_PID;
        NEXT_PID += 1;
        let mut i = 0;
        while i < MAX_PROCESSES {
            if PROCESSES[i].is_none() {
                PROCESSES[i] = Some(Process::new(pid, entry_point));
                PROCESS_COUNT += 1;
                return pid;
            }
            i += 1;
        }
        0
    }
}

pub fn get_process_count() -> usize {
    unsafe { PROCESS_COUNT }
}
