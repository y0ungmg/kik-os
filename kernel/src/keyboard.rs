// PS/2 Keyboard driver with scancode-to-ASCII conversion

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

// Scancode set 1 to ASCII (US QWERTY, unshifted)
const SCANCODE_TO_ASCII: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8, 9, // 0x00-0x0F
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 13, 0, b'a', b's', // 0x10-0x1F
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v', // 0x20-0x2F
    b'b', b'n', b'm', b',', b'.', b'/', 0, 0, 0, b' ', 0, 0, 0, 0, 0, 0, // 0x30-0x3F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x40-0x4F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x50-0x5F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x60-0x6F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x70-0x7F
];

// Shifted scancodes
const SCANCODE_TO_ASCII_SHIFT: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 8, 9, // 0x00-0x0F
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', 13, 0, b'A', b'S', // 0x10-0x1F
    b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~', 0, b'|', b'Z', b'X', b'C', b'V', // 0x20-0x2F
    b'B', b'N', b'M', b'<', b'>', b'?', 0, 0, 0, b' ', 0, 0, 0, 0, 0, 0, // 0x30-0x3F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x40-0x4F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x50-0x5F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x60-0x6F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x70-0x7F
];

static mut SHIFT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;
static mut INPUT_BUFFER: [u8; 256] = [0; 256];
static mut BUFFER_HEAD: usize = 0;
static mut BUFFER_TAIL: usize = 0;

pub fn init() {
    // Basic initialization
}

pub fn read_scancode() -> Option<u8> {
    unsafe {
        let status = inb(KEYBOARD_STATUS_PORT);
        if status & 0x01 != 0 {
            Some(inb(KEYBOARD_DATA_PORT))
        } else {
            None
        }
    }
}

pub fn process_scancode(sc: u8) {
    unsafe {
        // Handle key release (high bit set)
        if sc & 0x80 != 0 {
            let key = sc & 0x7F;
            if key == 0x2A || key == 0x36 { // Left/Right shift
                SHIFT_PRESSED = false;
            }
            return;
        }
        
        // Handle key press
        match sc {
            0x2A | 0x36 => { // Shift
                SHIFT_PRESSED = true;
            }
            0x3A => { // Caps Lock
                CAPS_LOCK = !CAPS_LOCK;
            }
            _ => {
                let shifted = SHIFT_PRESSED ^ CAPS_LOCK;
                let ascii = if shifted {
                    if (sc as usize) < 128 { SCANCODE_TO_ASCII_SHIFT[sc as usize] } else { 0 }
                } else {
                    if (sc as usize) < 128 { SCANCODE_TO_ASCII[sc as usize] } else { 0 }
                };
                if ascii != 0 {
                    // Add to buffer
                    let next_head = (BUFFER_HEAD + 1) % 256;
                    if next_head != BUFFER_TAIL {
                        INPUT_BUFFER[BUFFER_HEAD] = ascii;
                        BUFFER_HEAD = next_head;
                    }
                }
            }
        }
    }
}

pub fn get_char() -> Option<u8> {
    unsafe {
        if BUFFER_HEAD == BUFFER_TAIL {
            None
        } else {
            let c = INPUT_BUFFER[BUFFER_TAIL];
            BUFFER_TAIL = (BUFFER_TAIL + 1) % 256;
            Some(c)
        }
    }
}

pub fn has_input() -> bool {
    unsafe { BUFFER_HEAD != BUFFER_TAIL }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}