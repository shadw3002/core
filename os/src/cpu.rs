
// TODO: guest PRIMARY_HART_ID
pub const PRIMARY_HART_ID: usize = 0;

// tp is unused in kernel space
pub fn set_cpu_id(cpu_id: usize) {
    unsafe {
        core::arch::asm!("mv tp, {}", in(reg) cpu_id);
    }
}

pub fn id() -> usize {
    let cpu_id;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) cpu_id);
    }
    cpu_id
}

pub fn send_ipi(cpu_id: usize) {
    super::sbi::send_ipi(1 << cpu_id);
}

pub fn broadcast_ipi() {
    super::sbi::send_ipi(usize::max_value());
}

use core::sync::atomic::{AtomicBool, Ordering};
// TODO: force place in data segment but bss segment
// The data section contains all the initialized static variables with their initial value, 
// bss section contains all uninitialized/zero-initialized static variables
static STARTED: AtomicBool = AtomicBool::new(false);

pub fn wake_secondary_cpus() {
    // TODO:
    STARTED.store(true, Ordering::SeqCst);
}

pub fn is_primary_boot_done() -> bool {
    // TODO: 
    STARTED.load(Ordering::SeqCst)
}