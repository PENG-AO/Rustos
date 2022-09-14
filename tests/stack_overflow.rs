// stack_overflow.rs

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use core::panic::PanicInfo;
use rustos::{exit_qemu, QemuExitCode, serial_print, serial_println};

lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rustos::gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C"
fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    rustos::gdt::init();
    init_test_idt();
    stack_overflow();
    panic!("Execution continued after stack overflow");
}

extern "x86-interrupt"
fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _err_code: u64) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimization
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
