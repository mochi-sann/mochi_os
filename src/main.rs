#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock() , ", some numbers : {} {} "   ,50, 2.55).unwrap();
    write!(vga_buffer::WRITER.lock() , ",~\n\n\n some numbers : {} {} "   ,50, 2.55).unwrap();



    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
