// Process Scheduler - simple round-robin

// Removed unused imports

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

#[derive(Clone, Copy)]
pub struct Task {
    pub id: u64,
    pub context: TaskContext,
    pub state: TaskState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

static mut TASKS: [Option<Task>; 64] = [None; 64];
static mut TASK_COUNT: usize = 0;
static mut CURRENT_TASK: usize = 0;
static mut NEXT_TASK_ID: u64 = 1;

pub fn init() {
    // Scheduler initialized with no tasks
}

pub fn create_task(entry_point: u64) -> Option<u64> {
    unsafe {
        if TASK_COUNT >= 64 {
            return None;
        }
        
        let task_id = NEXT_TASK_ID;
        NEXT_TASK_ID += 1;
        
        let mut ctx = TaskContext {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: entry_point,
            rflags: 0x202, // Interrupts enabled
        };
        
        // Allocate stack
        if let Some(stack_base) = crate::pmm::alloc_page() {
            ctx.rsp = stack_base as u64 + 4096;
        } else {
            return None;
        }
        
        TASKS[TASK_COUNT] = Some(Task {
            id: task_id,
            context: ctx,
            state: TaskState::Ready,
        });
        TASK_COUNT += 1;
        
        Some(task_id)
    }
}

pub fn schedule() {
    unsafe {
        if TASK_COUNT == 0 {
            return;
        }
        
        // Find next ready task
        let start = CURRENT_TASK;
        loop {
            CURRENT_TASK = (CURRENT_TASK + 1) % TASK_COUNT;
            if let Some(ref task) = TASKS[CURRENT_TASK] {
                if task.state == TaskState::Ready {
                    break;
                }
            }
            if CURRENT_TASK == start {
                break; // No ready tasks
            }
        }
    }
}

pub fn get_current_task() -> Option<u64> {
    unsafe {
        if TASK_COUNT > 0 {
            if let Some(ref task) = TASKS[CURRENT_TASK] {
                return Some(task.id);
            }
        }
        None
    }
}