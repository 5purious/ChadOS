#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod screen;

static HELLO: &[u8] = b"Hello World!";


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! { 
    let mut screen = screen::Screen::new(0, screen::Color::Green);
    screen.writestr(&HELLO);
    loop {}
}
