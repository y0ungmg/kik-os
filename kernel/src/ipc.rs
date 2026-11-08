// Inter-Process Communication (IPC) - simple message passing

pub const MAX_MESSAGES: usize = 256;
pub const MAX_MESSAGE_SIZE: usize = 256;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Message {
    pub sender: u64,
    pub receiver: u64,
    pub msg_type: u32,
    pub size: usize,
    pub data: [u8; MAX_MESSAGE_SIZE],
}

static mut MESSAGE_QUEUE: [Option<Message>; MAX_MESSAGES] = [None; MAX_MESSAGES];
static mut QUEUE_HEAD: usize = 0;
static mut QUEUE_TAIL: usize = 0;
static mut QUEUE_COUNT: usize = 0;

pub fn send_message(msg: Message) -> bool {
    unsafe {
        if QUEUE_COUNT >= MAX_MESSAGES {
            return false;
        }
        MESSAGE_QUEUE[QUEUE_TAIL] = Some(msg);
        QUEUE_TAIL = (QUEUE_TAIL + 1) % MAX_MESSAGES;
        QUEUE_COUNT += 1;
        true
    }
}

pub fn receive_message(receiver: u64) -> Option<Message> {
    unsafe {
        if QUEUE_COUNT == 0 {
            return None;
        }
        
        // Find message for this receiver
        for i in 0..QUEUE_COUNT {
            let idx = (QUEUE_HEAD + i) % MAX_MESSAGES;
            if let Some(ref msg) = MESSAGE_QUEUE[idx] {
                if msg.receiver == receiver {
                    let msg = MESSAGE_QUEUE[idx].take().unwrap();
                    // Shift remaining messages
                    for j in i..QUEUE_COUNT - 1 {
                        let src = (QUEUE_HEAD + j + 1) % MAX_MESSAGES;
                        let dst = (QUEUE_HEAD + j) % MAX_MESSAGES;
                        MESSAGE_QUEUE[dst] = MESSAGE_QUEUE[src].take();
                    }
                    QUEUE_TAIL = (QUEUE_TAIL + MAX_MESSAGES - 1) % MAX_MESSAGES;
                    QUEUE_COUNT -= 1;
                    return Some(msg);
                }
            }
        }
        None
    }
}