// ATA/IDE storage driver (PIO mode)
// Supports primary master drive on I/O ports 0x1F0-0x1F7

use core::arch::asm;

const ATA_DATA: u16 = 0x1F0;
const ATA_ERROR: u16 = 0x1F1;
const ATA_SECTOR_COUNT: u16 = 0x1F2;
const ATA_LBA_LOW: u16 = 0x1F3;
const ATA_LBA_MID: u16 = 0x1F4;
const ATA_LBA_HIGH: u16 = 0x1F5;
const ATA_DRIVE: u16 = 0x1F6;
const ATA_STATUS: u16 = 0x1F7;
const ATA_COMMAND: u16 = 0x1F7;

const ATA_CMD_READ_PIO: u8 = 0x20;
const ATA_CMD_WRITE_PIO: u8 = 0x30;
const ATA_CMD_IDENTIFY: u8 = 0xEC;

const ATA_SR_BSY: u8 = 0x80; // Busy
const ATA_SR_DRDY: u8 = 0x40; // Drive ready
const ATA_SR_DRQ: u8 = 0x08; // Data request ready
const ATA_SR_ERR: u8 = 0x01; // Error

pub fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack));
    }
    val
}

pub fn outb(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
    }
}

pub fn inw(port: u16) -> u16 {
    let val: u16;
    unsafe {
        asm!("in ax, dx", out("ax") val, in("dx") port, options(nomem, nostack));
    }
    val
}

pub fn outw(port: u16, val: u16) {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") val, options(nomem, nostack));
    }
}

fn wait_bsy() {
    while inb(ATA_STATUS) & ATA_SR_BSY != 0 {}
}

fn wait_drq() {
    while inb(ATA_STATUS) & ATA_SR_DRQ == 0 {}
}

pub fn init() {
    // Select primary master drive
    outb(ATA_DRIVE, 0xA0);
    // Wait for drive to be ready
    wait_bsy();
    // Send IDENTIFY command
    outb(ATA_COMMAND, ATA_CMD_IDENTIFY);
    // Wait for response
    wait_bsy();
    let status = inb(ATA_STATUS);
    if status == 0 {
        // No drive present
        return;
    }
    // Wait for DRQ
    wait_drq();
    // Read 256 words (512 bytes) of identify data
    let mut i = 0;
    while i < 256 {
        let _ = inw(ATA_DATA);
        i += 1;
    }
}

pub fn read_sector(lba: u32, buffer: &mut [u8; 512]) {
    wait_bsy();
    outb(ATA_DRIVE, 0xE0 | ((lba >> 24) & 0x0F) as u8);
    outb(ATA_SECTOR_COUNT, 1);
    outb(ATA_LBA_LOW, (lba & 0xFF) as u8);
    outb(ATA_LBA_MID, ((lba >> 8) & 0xFF) as u8);
    outb(ATA_LBA_HIGH, ((lba >> 16) & 0xFF) as u8);
    outb(ATA_COMMAND, ATA_CMD_READ_PIO);
    wait_bsy();
    wait_drq();
    // Read 256 words
    let mut i = 0usize;
    while i < 256 {
        let word = inw(ATA_DATA);
        buffer[i * 2] = (word & 0xFF) as u8;
        buffer[i * 2 + 1] = ((word >> 8) & 0xFF) as u8;
        i += 1;
    }
}

pub fn write_sector(lba: u32, buffer: &[u8; 512]) {
    wait_bsy();
    outb(ATA_DRIVE, 0xE0 | ((lba >> 24) & 0x0F) as u8);
    outb(ATA_SECTOR_COUNT, 1);
    outb(ATA_LBA_LOW, (lba & 0xFF) as u8);
    outb(ATA_LBA_MID, ((lba >> 8) & 0xFF) as u8);
    outb(ATA_LBA_HIGH, ((lba >> 16) & 0xFF) as u8);
    outb(ATA_COMMAND, ATA_CMD_WRITE_PIO);
    wait_bsy();
    wait_drq();
    // Write 256 words
    let mut i = 0usize;
    while i < 256 {
        let word = (buffer[i * 2] as u16) | ((buffer[i * 2 + 1] as u16) << 8);
        outw(ATA_DATA, word);
        i += 1;
    }
    // Flush cache
    outb(ATA_COMMAND, 0xE7); // CACHE FLUSH
    wait_bsy();
}