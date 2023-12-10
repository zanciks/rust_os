// to run the code, my IDE (RustRover) simply does
// $cargo bootimage
// $/bin/bash ./qemu.sh

// disable the standard library, as it is OS-based
#![no_std]
// we cant have a main function. instead, we define _start()
#![no_main]
// we need to be able to deal with panics
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";


// by using no_mangle, the function will actually be named _start() when compiled
// (otherwise, it would be named something random/mangled)
// this is needed, as machine code uses _start to begin its code
// pub extern C just means that we use the C naming convention, instead of Rust's.
// we should never return from this function, so we a "returning" the never type
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// again, we return the never type, as when we panic, we don't return.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}