#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate os;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(_tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}