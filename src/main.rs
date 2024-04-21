#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

mod vga_buffer;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// This function will be called when `panic`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
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
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");

    loop {}
}
