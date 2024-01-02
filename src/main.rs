#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[allow(unused_imports)]
use my_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world");
    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

// Panic handler other than test modes
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}
