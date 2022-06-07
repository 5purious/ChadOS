#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct Screen {
    x: usize,
    draw_color: Color,
    vga: *mut u8
}


impl Screen {
    pub fn new(ix: usize, draw_color: Color) -> Self {
        Self { x: ix, draw_color: draw_color, vga: 0xB8000 as *mut u8 }
    }

    pub fn writestr(&mut self, str: &[u8]) {
        for &byte in str.iter() {
            unsafe {
                *self.vga.offset(self.x as isize * 2) = byte;
                *self.vga.offset(self.x as isize * 2 + 1) = self.draw_color as u8;
            }

            self.x += 1;
        }
    }
}
