// 禁止使用 Rust 标准库 std
#![no_std]
// start 语义项代表了标准库 std 在执行应用程序之前需要进行的一些初始化工作
// 在失去了 main 函数的情况下，编译器也就不需要完成所谓的初始化工作了。
#![no_main]
// 通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]
#![feature(core_panic)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

#[macro_use]
extern crate bitflags;
extern crate alloc;


#[macro_use]
mod console;
mod sbi;
mod panic_handler;
mod logging;
mod mem;
mod config;
mod sync;
mod cpu;

use log::{error, info, warn};
use core::arch::global_asm;

const PRIMARY_HART_ID: usize = 0;

global_asm!(include_str!("entry.asm"));

#[no_mangle] // 不混淆符号名
pub fn main_dispatcher(hart_id: usize, _device_tree_paddr: usize) -> ! {
    unsafe {
        cpu::set_cpu_id(hartid);
    }
    
    // TODO: guest PRIMARY_HART_ID
    if hartid == PRIMARY_HART_ID {
        primary_boot();
    } else {
        secondary_boot();
    }

    // TODO: main loop
    // loop{}

    sbi::shutdown();
}

fn init() {
    info!("initialization starting:");
    // mem::init();
    logging::init();
    info!("initialization done.");
}

fn primary_boot() {
    init();
    mem::log_memory_space();
    info!("hello, world!");
}

fn secondary_boot() {

}