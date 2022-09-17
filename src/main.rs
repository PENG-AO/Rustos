// main.rs

#![no_std] // block standard lib
#![no_main] // reset entry point
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" // entry point for cargo run
fn _start() -> ! {
    rustos::init();

    println!("Hello World! at {}", "15:28");

    #[cfg(test)]
    test_main();

    rustos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler] // normal one
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
