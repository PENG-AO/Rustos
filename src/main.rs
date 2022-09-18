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
    // use rustos::memory::active_level4_table;
    // use x86_64::VirtAddr;

    println!("Hello World!");
    rustos::init();

    // let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // println!("here1");
    // let level4_table = unsafe { active_level4_table(physical_mem_offset) };
    // println!("here2");

    // for (idx, entry) in level4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 entry {}: {:?}", idx, entry);
    //     }
    // }

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
