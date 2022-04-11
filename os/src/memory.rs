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

fn init_bss() {
    (s_bss as usize..e_bss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
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

pub const MEMORY_END: usize = 0x80800000;
