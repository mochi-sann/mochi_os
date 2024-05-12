#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga_buffer::print_something;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {

    use core::fmt::Write; 

    println!("Hello World{}", "!");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
