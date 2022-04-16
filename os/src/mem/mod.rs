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
pub use memory_set::{MapPermission, MemorySet};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};
use spin::Mutex;
use alloc::sync::Arc;

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

    /// a memory set instance through lazy_static! managing kernel space
    pub static ref KERNEL_SPACE: Arc<Mutex<MemorySet>> =
        Arc::new(unsafe { Mutex::new(new_kernel_space()) });
}

pub const MEMORY_END: usize = 0x80800000;

pub fn clean_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(
            *S_BSS as *mut u8, 
            *E_BSS - *S_BSS
        ).fill(0);
    }
}

pub fn log_memory_space() {
    // TODO: entirely
    info!("[memory layout]");
    info!("+---kernel  base address {:#x}", s_kernel as usize);
    info!("|---.text   [{:#x}, {:#x})", s_text as usize, e_text as usize);
    info!("|---.rodata [{:#x}, {:#x})", s_rodata as usize, e_rodata as usize);
    info!("|---.data   [{:#x}, {:#x})", s_data as usize, e_data as usize);
    info!("|    +----  stack   [{:#x}, {:#x})", *BOOT_STACK, *BOOT_STACK_TOP);
    info!("|---.bss    [{:#x}, {:#x})", s_bss as usize, e_bss as usize);
    info!("|---kernel  end  address {:#x}", e_kernel as usize);
    info!("+---memory  end  address {:#x}", MEMORY_END);
    
}

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    log_memory_space();

    heap_allocator::init();
    frame_allocator::init();
    {
        KERNEL_SPACE.lock().activate();
        info!("[kernel space] init done.");
    }
}

/// Without kernel stacks.
fn new_kernel_space() -> MemorySet {
    use memory_set::{MapType, MapArea};

    let mut memory_set = MemorySet::new_bare();
    // map trampoline
    memory_set.map_trampoline();
    // map kernel sections
    // info!("mapping .text section");
    memory_set.push(
        MapArea::new(
            (*S_TEXT).into(),
            (*E_TEXT).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::X,
        ),
        None,
    );
    // info!("mapping .rodata section");
    memory_set.push(
        MapArea::new(
            (*S_RODATA).into(),
            (*E_RODATA).into(),
            MapType::Identical,
            MapPermission::R,
        ),
        None,
    );
    // info!("mapping .data section");
    memory_set.push(
        MapArea::new(
            (*S_DATA).into(),
            (*E_DATA).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );
    // info!("mapping .bss section");
    memory_set.push(
        MapArea::new(
            (*S_BSS).into(),
            (*E_BSS).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );
    // info!("mapping physical memory");
    memory_set.push(
        MapArea::new(
            (*E_KERNEL).into(),
            MEMORY_END.into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );
    // kernel_space_remap_test();
    memory_set
}

#[allow(unused)]
pub fn kernel_space_remap_test() {
    let mut kernel_space = KERNEL_SPACE.lock();
    let mid_text: VirtAddr = ((*S_TEXT + *E_TEXT) / 2).into();
    let mid_rodata: VirtAddr = ((*S_RODATA + *E_RODATA) / 2).into();
    let mid_data: VirtAddr = ((*S_DATA + *E_DATA) / 2).into();
    assert!(
        !kernel_space
            .page_table
            .translate(mid_text.floor())
            .unwrap()
            .writable(),
    );
    assert!(
        !kernel_space
            .page_table
            .translate(mid_rodata.floor())
            .unwrap()
            .writable(),
    );
    assert!(
        !kernel_space
            .page_table
            .translate(mid_data.floor())
            .unwrap()
            .executable(),
    );
    // info!("remap_test passed!");
}
