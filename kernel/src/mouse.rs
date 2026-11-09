// PS/2 Mouse driver (basic)
// Uses the same controller as keyboard (port 0x60/0x64)

const MOUSE_DATA_PORT: u16 = 0x60;
const MOUSE_STATUS_PORT: u16 = 0x64;
const MOUSE_COMMAND_PORT: u16 = 0x64;

// Mouse state
static mut MOUSE_X: i32 = 400; // Center of 800x600 (but we use 1280x800)
static mut MOUSE_Y: i32 = 400;
static mut MOUSE_BUTTONS: u8 = 0;
static mut MOUSE_PACKET: [u8; 3] = [0; 3];
static mut MOUSE_PACKET_IDX: usize = 0;
static mut MOUSE_READY: bool = false;

pub fn init() {
    unsafe {
        crate::serial::print("Mouse init start.\n");
        // Enable auxiliary device (mouse)
        wait_write();
        outb(MOUSE_COMMAND_PORT, 0xA8); // Enable auxiliary device
        
        crate::serial::print("Enabling data reporting...\n");
        // Set defaults
        wait_write();
        outb(MOUSE_COMMAND_PORT, 0xD4); // Write to mouse
        wait_write();
        outb(MOUSE_DATA_PORT, 0xF6); // Set defaults
        wait_read();
        let _ = inb(MOUSE_DATA_PORT); // ACK
        
        // Enable data reporting
        wait_write();
        outb(MOUSE_COMMAND_PORT, 0xD4); // Write to mouse
        wait_write();
        outb(MOUSE_DATA_PORT, 0xF4); // Enable data reporting
        wait_read();
        let _ = inb(MOUSE_DATA_PORT); // ACK
        
        MOUSE_READY = true;
        crate::serial::print("Mouse init done.\n");
    }
}

pub fn process_byte(byte: u8) {
    unsafe {
        if !MOUSE_READY {
            return;
        }
        
        MOUSE_PACKET[MOUSE_PACKET_IDX] = byte;
        MOUSE_PACKET_IDX += 1;
        
        if MOUSE_PACKET_IDX >= 3 {
            // Process complete packet
            let flags = MOUSE_PACKET[0];
            let dx = MOUSE_PACKET[1] as i8 as i32;
            let dy = MOUSE_PACKET[2] as i8 as i32;
            
            MOUSE_BUTTONS = flags & 0x07;
            
            // Update position
            MOUSE_X += dx;
            MOUSE_Y -= dy; // Y is inverted
            
            // Clamp to screen bounds
            if MOUSE_X < 0 { MOUSE_X = 0; }
            if MOUSE_X > 1279 { MOUSE_X = 1279; }
            if MOUSE_Y < 0 { MOUSE_Y = 0; }
            if MOUSE_Y > 799 { MOUSE_Y = 799; }
            
            MOUSE_PACKET_IDX = 0;
        }
    }
}

pub fn get_x() -> i32 {
    unsafe { MOUSE_X }
}

pub fn get_y() -> i32 {
    unsafe { MOUSE_Y }
}

pub fn get_buttons() -> u8 {
    unsafe { MOUSE_BUTTONS }
}

pub fn is_ready() -> bool {
    unsafe { MOUSE_READY }
}

fn wait_write() {
    unsafe {
        let mut timeout = 1000000u32;
        while timeout > 0 {
            if inb(MOUSE_STATUS_PORT) & 0x02 == 0 {
                return;
            }
            timeout -= 1;
        }
    }
}

fn wait_read() {
    unsafe {
        let mut timeout = 1000000u32;
        while timeout > 0 {
            if inb(MOUSE_STATUS_PORT) & 0x01 != 0 {
                return;
            }
            timeout -= 1;
        }
    }
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack));
    }
    val
}

unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
    }
}