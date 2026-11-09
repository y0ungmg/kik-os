// Compositor - manages multiple windows with z-ordering and redraw
// Builds on the framebuffer and window modules

use crate::framebuffer::Framebuffer;
use crate::window::Window;

pub const MAX_WINDOWS: usize = 4;

pub struct Compositor {
    pub fb: Framebuffer,
    pub windows: [Option<Window>; MAX_WINDOWS],
    pub window_count: usize,
    pub focused: usize,
}

impl Compositor {
    pub fn new(fb: Framebuffer) -> Self {
        Self {
            fb,
            windows: [None; MAX_WINDOWS],
            window_count: 0,
            focused: 0,
        }
    }

    pub fn add_window(&mut self, win: Window) -> bool {
        if self.window_count >= MAX_WINDOWS {
            return false;
        }
        self.windows[self.window_count] = Some(win);
        self.window_count += 1;
        self.focused = self.window_count - 1;
        true
    }

    pub fn draw_all(&mut self) {
        // Draw desktop background (skip full clear for performance)
        // Only draw taskbar at bottom
        let mut ty = 760usize;
        while ty < 800 {
            let mut tx = 0usize;
            while tx < 1280 {
                self.fb.set_pixel(tx as u32, ty as u32, 0x00202030);
                tx += 1;
            }
            ty += 1;
        }
        
        // Draw clock area on taskbar (right side)
        // Simple: draw a small rectangle for clock
        let mut cy = 765usize;
        while cy < 795 {
            let mut cx = 1180usize;
            while cx < 1270 {
                self.fb.set_pixel(cx as u32, cy as u32, 0x00303040);
                cx += 1;
            }
            cy += 1;
        }
        
        // Draw windows
        let mut i = 0usize;
        while i < self.window_count {
            if let Some(ref win) = self.windows[i] {
                win.draw(self.fb.address, self.fb.pitch);
            }
            i += 1;
        }
    }

    pub fn draw_window(&self, index: usize) {
        if index < self.window_count {
            if let Some(ref win) = self.windows[index] {
                win.draw(self.fb.address, self.fb.pitch);
            }
        }
    }
}