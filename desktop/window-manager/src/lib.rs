#![no_std]

// KiK Desktop - Window Manager (basic structure)

#[derive(Clone, Copy)]
pub struct Window {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
}

pub struct WindowManager {
    pub windows: [Option<Window>; 64],
    pub window_count: usize,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: [None; 64],
            window_count: 0,
        }
    }
    
    pub fn create_window(&mut self, x: i32, y: i32, width: u32, height: u32, title: &'static str) -> Option<usize> {
        if self.window_count >= 64 {
            return None;
        }
        let idx = self.window_count;
        self.windows[idx] = Some(Window { x, y, width, height, title });
        self.window_count += 1;
        Some(idx)
    }
    
    pub fn close_window(&mut self, idx: usize) {
        if idx < 64 {
            self.windows[idx] = None;
        }
    }
}