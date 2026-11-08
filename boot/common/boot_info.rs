// KiK OS Boot Information Structure
// Shared between bootloader and kernel

#![no_std]

/// Boot information passed from bootloader to kernel
#[repr(C)]
pub struct BootInfo {
    /// Magic number to verify boot info validity
    pub magic: u64,
    /// Kernel load address
    pub kernel_load_addr: u64,
    /// Kernel size in bytes
    pub kernel_size: u64,
    /// Physical memory map address
    pub memory_map_addr: u64,
    /// Number of memory map entries
    pub memory_map_count: u64,
    /// Framebuffer base address
    pub framebuffer_addr: u64,
    /// Framebuffer width
    pub framebuffer_width: u32,
    /// Framebuffer height
    pub framebuffer_height: u32,
    /// Framebuffer pitch (bytes per scanline)
    pub framebuffer_pitch: u32,
    /// Framebuffer bits per pixel
    pub framebuffer_bpp: u8,
    /// Boot mode: 0 = BIOS, 1 = UEFI
    pub boot_mode: u8,
}

pub const BOOT_INFO_MAGIC: u64 = 0x4B694B4F53424F4F; // "KiKOSBOO" in hex-ish
pub const BOOT_MODE_BIOS: u8 = 0;
pub const BOOT_MODE_UEFI: u8 = 1;