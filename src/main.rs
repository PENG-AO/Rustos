// main.rs

#![no_std] // block standard lib
#![no_main] // reset entry point

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (idx, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(idx as isize * 2) = byte;
            *vga_buffer.offset(idx as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
