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
        // Draw windows only (skip taskbar for performance)
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