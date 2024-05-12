#![no_std]
#![no_main]

use core::panic::PanicInfo;

use vga_buffer::print_something;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!!!!!!!!!!!!!!!!!!!!!!!!!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    use core::fmt::Write; 
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    write!(vga_buffer::WRITER.lock(), ",\n some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
