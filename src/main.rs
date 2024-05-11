#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use std::{print, println};

static HELLO: &[u8] = b"Hello World!";

#[reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("hello world ");
    for i in 0..5 {
        println!("Hello World{} {}", "!", i);
    }

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(" panic {} ", info);
    loop {}
}

#![feature(custom_test_frameworks)]
#[test_runner(crate::test_runner)]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}




#[test_case]
fn trivial_assertion() {
    print!("trivial assertion ... " );
    assert_eq!(1, 1);
    println!("[OK]")
}
