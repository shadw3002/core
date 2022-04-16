// 禁止使用 Rust 标准库 std
#![no_std]
// start 语义项代表了标准库 std 在执行应用程序之前需要进行的一些初始化工作
// 在失去了 main 函数的情况下，编译器也就不需要完成所谓的初始化工作了。
#![no_main]
// 通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]
#![feature(core_panic)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate bitflags;
extern crate alloc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod console;
mod sbi;
mod panic_handler;
mod logging;
mod mem;
mod config;
mod sync;
mod cpu;
mod loader;



const PRIMARY_HART_ID: usize = 0;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle] // 不混淆符号名
pub fn main_dispatcher(hart_id: usize, _device_tree_paddr: usize) -> ! {
    unsafe { cpu::set_cpu_id(hart_id); }

    static EARLY_BLOCK_INIT: spin::Once<()> = spin::Once::new();
    EARLY_BLOCK_INIT.call_once(||{
        // .bss must be cleaned firstly
        mem::clean_bss();
        logging::init();
    });

    // TODO: guest PRIMARY_HART_ID
    if hart_id == PRIMARY_HART_ID {
        primary_boot();
    } else {
        secondary_boot();
    }
    

    // TODO: main loop
    // loop{}

    sbi::shutdown();
}


fn primary_boot() {
    info!("primary_boot starting...");

    mem::init();
    
    info!("primary_boot done.");
}

fn secondary_boot() {
    info!("secondary_boot starting...");

    info!("secondary_boot done.");

    let mut x = 1;
    loop{
        x += 1;
    }
}