// 禁止使用 Rust 标准库 std
#![no_std]
// start 语义项代表了标准库 std 在执行应用程序之前需要进行的一些初始化工作
// 在失去了 main 函数的情况下，编译器也就不需要完成所谓的初始化工作了。
#![no_main]
// 通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]
#![feature(core_panic)]
#![feature(alloc_error_handler)]

#![feature(atomic_from_mut)]
#![feature(const_mut_refs)]

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
mod trap;
mod task;

use core::{arch::global_asm, panicking::panic};
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle] // 不混淆符号名
fn main_dispatcher(hart_id: usize, _device_tree_paddr: usize) -> ! {
    cpu::set_cpu_id(hart_id);

    match hart_id {
        cpu::PRIMARY_HART_ID => primary_boot(hart_id),
        _                    => secondary_boot(hart_id),
    }

    sbi::shutdown();
}


fn primary_boot(hart_id: usize) {
    // .bss must be cleaned firstly
    mem::clean_bss();
    logging::init();

    info!("primary cpu {} booting...", hart_id);

    // mem::init();
    
    cpu::wake_secondary_cpus();

    info!("primary cpu {} booting done.", hart_id);
}

fn secondary_boot(hart_id: usize) {
    while !cpu::is_primary_boot_done() {}

    info!("secondary cpu {} booting...", hart_id);

    // mem::init();
    
    info!("secondary cpu {} booting done.", hart_id);
}