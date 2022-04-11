use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler] // 一种编译指导属性，用于标记核心库core中的 panic! 宏要对接的函数
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
