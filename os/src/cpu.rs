// tp is unused in kernel space
pub unsafe fn set_cpu_id(cpu_id: usize) {
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
