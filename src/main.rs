// to run the code, my IDE (RustRover) simply does
// $cargo bootimage
// $/bin/bash ./qemu.sh

// disable the standard library, as it is OS-based
#![no_std]
// we cant have a main function. instead, we define _start()
#![no_main]
// custom tests
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use rust_os::println;
use core::panic::PanicInfo;

// by using no_mangle, the function will actually be named _start() when compiled
// (otherwise, it would be named something random/mangled)
// this is needed, as machine code uses _start to begin its code
// pub extern C just means that we use the C naming convention, instead of Rust's.
// we should never return from this function, so we a "returning" the never type
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main(); // we call this because we declared it our main test function (line 12)

    loop {}
}

// again, we return the never type, as when we panic, we don't return.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// we use another panic function for when we're in test mode, because we want to use
// serial print instead of print, as we want the PanicInfo on our host computer
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
