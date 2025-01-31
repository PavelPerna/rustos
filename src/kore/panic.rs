use core::panic::PanicInfo;
#[panic_handler]
#[cfg(not(test))]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}