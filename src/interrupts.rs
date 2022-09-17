// interrupts.rs

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
use pic8259::ChainedPics;
use spin;
use lazy_static::lazy_static;
use crate::print;
use crate::println;
use crate::gdt;
use crate::hlt_loop;

// PIC layout           ____________                          ____________
// Real Time Clock --> |            |   Timer -------------> |            |
// ACPI -------------> |            |   Keyboard-----------> |            |      _____
// Available --------> | Secondary  |----------------------> | Primary    |     |     |
// Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
// Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
// Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
// Primary ATA ------> |            |   Floppy disk -------> |            |
// Secondary ATA ----> |____________|   Parallel Port 1----> |____________|
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }
    fn as_usize(self) -> usize { usize::from(self.as_u8()) }
}

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_IDX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt"
fn double_fault_handler(stack_frame: InterruptStackFrame, _err_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

fn send_eoi(interrupt_idx: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt_idx.as_u8());
    }
}

extern "x86-interrupt"
fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    send_eoi(InterruptIndex::Timer);
}

extern "x86-interrupt"
fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use spin::Mutex;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

    lazy_static!{
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }
    
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(char) => print!("{}", char),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    send_eoi(InterruptIndex::Keyboard);
}

extern "x86-interrupt"
fn page_fault_handler(stack_frame: InterruptStackFrame, err_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Adress: {:?}", Cr2::read());
    println!("Error Code: {:?}", err_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
