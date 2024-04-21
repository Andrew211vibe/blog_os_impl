#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This function will be called when `panic`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
//    let vga_buf = 0xb8000 as *mut u8;
    
//    for (i, &byte) in HELLO.iter().enumerate() {
//        unsafe {
//            *vga_buf.offset(i as isize * 2) = byte;
//            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
//        }
//    }
//    vga_buffer::print_something();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
