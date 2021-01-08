#![no_std]
#![no_main]
#![feature(global_asm)]
global_asm!(include_str!("entry.asm"));
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {}
}
