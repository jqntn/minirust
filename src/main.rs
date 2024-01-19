#![no_std]
#![no_main]

#[no_mangle]
extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
