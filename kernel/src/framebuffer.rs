// Framebuffer driver (VESA/VBE compatible)

pub struct Framebuffer {
    pub address: u64,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bpp: u8,
}

impl Framebuffer {
    pub fn new(address: u64, width: u32, height: u32, pitch: u32, bpp: u8) -> Self {
        Self { address, width, height, pitch, bpp }
    }
    
    pub fn set_pixel(&self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let offset = (y * self.pitch + x * (self.bpp as u32 / 8)) as usize;
        let ptr = (self.address as usize + offset) as *mut u32;
        unsafe {
            *ptr = color;
        }
    }
    
    pub fn clear(&self, color: u32) {
        let mut y = 0;
        while y < self.height {
            let mut x = 0;
            while x < self.width {
                self.set_pixel(x, y, color);
                x += 1;
            }
            y += 1;
        }
    }
}