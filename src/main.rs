// to run the code, my IDE (RustRover) simply does
// $cargo bootimage
// $/bin/bash ./qemu.sh

// disable the standard library, as it is OS-based
#![no_std]
// we cant have a main function. instead, we define _start()
#![no_main]
// we need to be able to deal with panics
use core::panic::PanicInfo;

mod vga_buffer;

// by using no_mangle, the function will actually be named _start() when compiled
// (otherwise, it would be named something random/mangled)
// this is needed, as machine code uses _start to begin its code
// pub extern C just means that we use the C naming convention, instead of Rust's.
// we should never return from this function, so we a "returning" the never type
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("This text is on a new line!");
    print!("This is a number {}", 724);

    loop {}
}

// again, we return the never type, as when we panic, we don't return.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}