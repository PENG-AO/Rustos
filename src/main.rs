// main.rs

#![no_std] // block standard lib
#![no_main] // reset entry point

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("hello ").unwrap();
    write!(vga_buffer::WRITER.lock(), "again at {}", "16:15").unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
