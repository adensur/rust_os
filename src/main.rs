#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gaiduk_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gaiduk_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    gaiduk_os::init();
    println!("Hello World{}", "!");
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gaiduk_os::test_panic_handler(info)
}
