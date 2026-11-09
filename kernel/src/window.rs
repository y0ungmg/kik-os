// Simple window manager

#[derive(Clone, Copy)]
pub struct Window {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
}

impl Window {
    pub fn new(x: u32, y: u32, width: u32, height: u32, title: &'static str) -> Self {
        Self { x, y, width, height, title }
    }
    
    pub fn draw(&self, fb_addr: u64, pitch: u32) {
        // Draw window background (white)
        let mut wy = self.y;
        while wy < self.y + self.height {
            let mut wx = self.x;
            while wx < self.x + self.width {
                let offset = (wy * pitch + wx * 4) as usize;
                let ptr = (fb_addr as usize + offset) as *mut u32;
                unsafe { *ptr = 0x00FFFFFF; }
                wx += 1;
            }
            wy += 1;
        }
        
        // Draw title bar (blue) - 15px high
        let mut ty = self.y;
        while ty < self.y + 15 {
            let mut tx = self.x;
            while tx < self.x + self.width {
                let offset = (ty * pitch + tx * 4) as usize;
                let ptr = (fb_addr as usize + offset) as *mut u32;
                unsafe { *ptr = 0x000000FF; }
                tx += 1;
            }
            ty += 1;
        }
    }
}