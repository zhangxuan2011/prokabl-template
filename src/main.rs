//! The Proka Bootloader Template

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(self::test_runner)]
#![reexport_test_harness_main = "test_main"]

use proka_bootloader::BootInfo;
use core::panic::PanicInfo;

// Panic handler
#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub extern "C" fn kernel_main(info: &BootInfo) -> ! {
    let info = *info;
    let framebuffer = info.framebuffer();
    unsafe {
        let ptr = framebuffer.address() as *mut u8;
        for i in 0..500 {
            let offset = framebuffer.pitch() * i + i * framebuffer.bpp();
            ptr.add(offset as usize).cast::<u32>().write(0x00FFFFFF);
        }
    }
    loop {}
}

// Test runner 
#[cfg(test)]
fn test_runner(tests: &[&'static dyn Fn()]) {
    for test in tests {
        test();
    }
}