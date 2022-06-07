pub struct Screen {
    x: usize,
    draw_color: u8,
    vga: *mut u8
}


impl Screen {
    pub fn new(ix: usize, draw_color: u8) -> Self {
        Self { x: ix, draw_color: draw_color, vga: 0xB8000 as *mut u8 }
    }

    pub fn writestr(&mut self, str: &[u8]) {
        for &byte in str.iter() {
            unsafe {
                *self.vga.offset(self.x as isize * 2) = byte;
                *self.vga.offset(self.x as isize * 2 + 1) = self.draw_color;
            }

            self.x += 1;
        }
    }
}
