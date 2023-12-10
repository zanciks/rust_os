// to run the code, my IDE (RustRover) simply does
// $cargo bootimage
// $/bin/bash ./qemu.sh

// disable the standard library, as it is OS-based
#![no_std]
// we cant have a main function. instead, we define _start()
#![no_main]
// custom tests
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
// we need to be able to deal with panics
use core::panic::PanicInfo;
use x86_64;

mod vga_buffer;
mod serial;

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // exit qemu
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
// we number the enums because we should define our own error codes
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}