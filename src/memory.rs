// memory.rs

use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;
use x86_64::PhysAddr;

pub unsafe fn active_level4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level4_table_frame, _) = Cr3::read();
    let physical_addr = level4_table_frame.start_address();
    let virtual_addr = physical_mem_offset + physical_addr.as_u64();
    let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();
    
    &mut *page_table_ptr
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_mem_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level4_table_frame, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index()
    ];
    let mut frame = level4_table_frame;

    for &index in &table_indexes {
        let virtual_addr = physical_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virtual_addr.as_ptr();
        let table = unsafe { &*table_ptr };
        let entry = &table[index];

        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported")
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}
