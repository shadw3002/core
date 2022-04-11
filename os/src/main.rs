// 禁止使用 Rust 标准库 std
#![no_std]
// start 语义项代表了标准库 std 在执行应用程序之前需要进行的一些初始化工作
// 在失去了 main 函数的情况下，编译器也就不需要完成所谓的初始化工作了。
#![no_main]
// 通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]
#![feature(core_panic)]

mod sbi;
#[macro_use]
mod console;
mod panic_handler;
mod logging;
mod mem;
mod config;

use log::{error, info, warn};


use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

extern crate alloc;

fn init() {
    info!("initialization starting:");
    mem::init();
    logging::init();
    info!("initialization done.");
}


#[no_mangle] // 不混淆符号名
pub fn primary_main() -> ! {
    init();
    mem::log_memory_space();
    info!("hello, world!");

    sbi::shutdown();
}


   