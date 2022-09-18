// memory.rs

use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

pub unsafe fn active_level4_table(physical_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level4_table_frame, _) = Cr3::read();
    let physical_addr = level4_table_frame.start_address();
    let virtual_addr = physical_mem_offset + physical_addr.as_u64();
    let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();
    
    &mut *page_table_ptr
}
