//! Memory management implementation
//! 
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//! 
//! Every task or process has a memory_set to control its virtual memory.


mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

extern "C" {
    fn s_bss();
    fn e_bss();
    fn s_text();
    fn e_text();
    fn s_rodata();
    fn e_rodata();
    fn s_data();
    fn e_data();
    fn s_kernel();
    fn e_kernel();
    fn s_trampoline();
    fn boot_stack();
    fn boot_stack_top();
}

lazy_static! {
    pub static ref S_BSS: usize = s_bss as usize;
    pub static ref E_BSS: usize = e_bss as usize;
    pub static ref S_TEXT: usize = s_text as usize;
    pub static ref E_TEXT: usize = e_text as usize;
    pub static ref S_RODATA: usize = s_rodata as usize;
    pub static ref E_RODATA: usize = e_rodata as usize;
    pub static ref S_DATA: usize = s_data as usize;
    pub static ref E_DATA: usize = e_data as usize;
    pub static ref S_KERNEL: usize = s_kernel as usize;
    pub static ref E_KERNEL: usize = e_kernel as usize;
    pub static ref S_TRAMPOLINE: usize = s_trampoline as usize;
    pub static ref BOOT_STACK: usize = boot_stack as usize;
    pub static ref BOOT_STACK_TOP: usize = boot_stack_top as usize;
}

pub const MEMORY_END: usize = 0x80800000;

pub fn clean_bss() {
    info!("cleaning");
    unsafe {
        core::slice::from_raw_parts_mut(
            *S_BSS as *mut u8, 
            *E_BSS - *S_BSS
        ).fill(0);
    }
    info!("clean done");
}



pub fn log_memory_space() {
    info!("kernel  base address {:#x}", s_kernel as usize);
    info!(".text   [{:#x}, {:#x})", s_text as usize, e_text as usize);
    info!(".rodata [{:#x}, {:#x})", s_rodata as usize, e_rodata as usize);
    info!(".data   [{:#x}, {:#x})", s_data as usize, e_data as usize);
    info!(".bss    [{:#x}, {:#x})", s_bss as usize, e_bss as usize);
    info!("kernel  end  address {:#x}", e_kernel as usize);

    info!("stack [{:#x}, {:#x})", *BOOT_STACK, *BOOT_STACK_TOP);
}


/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    // frame_allocator::init_frame_allocator();
    // KERNEL_SPACE.exclusive_access().activate();

    log_memory_space();
}
