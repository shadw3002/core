

mod address;
mod frame_allocator;

pub use address::{PhysPageNum};


use log::info;

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
}

use lazy_static::lazy_static;
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
}

pub const MEMORY_END: usize = 0x80800000;

fn init_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(*S_BSS as *mut u8, *E_BSS - *S_BSS)
            .fill(0);
    }
}

pub fn init() {
    init_bss()
}

pub fn log_memory_space() {
    info!("kernel  base address {:#x}", s_kernel as usize);
    info!(".text   [{:#x}, {:#x})", s_text as usize, e_text as usize);
    info!(".rodata [{:#x}, {:#x})", s_rodata as usize, e_rodata as usize);
    info!(".data   [{:#x}, {:#x})", s_data as usize, e_data as usize);
    info!(".bss    [{:#x}, {:#x})", s_bss as usize, e_bss as usize);
    info!("kernel  end  address {:#x}", e_kernel as usize);
}


