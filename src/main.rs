// main.rs

#![no_std] // block standard lib
#![no_main] // reset entry point
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use bootloader::entry_point;
use rustos::println;

entry_point!(kernel_main); // setup entry point with type-checked

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustos::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World!");
    rustos::init();

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        0xb8000, // the identity-mapped vga buffer page
        0x201008, // some code page
        0x0100_0020_1a10, // some stack page
        boot_info.physical_memory_offset, // virtual address mapped to physical address 0
    ];

    for &address in &addresses {
        let virtual_addr = VirtAddr::new(address);
        let physical_addr = unsafe { translate_addr(virtual_addr, physical_mem_offset) };
        println!("{:?} -> {:?}", virtual_addr, physical_addr);
    }

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
