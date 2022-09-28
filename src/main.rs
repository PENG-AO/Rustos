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

extern crate alloc;
use alloc::boxed::Box;

entry_point!(kernel_main); // setup entry point with type-checked

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustos::memory;
    use rustos::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    println!("Hello World!");
    rustos::init();

    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e) };

    let x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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
