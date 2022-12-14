#![no_std]
#![no_main]
#![feature(lang_items)]

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub fn ignored() {}