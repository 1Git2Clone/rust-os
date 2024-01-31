// src/main.rs

#![no_std] // excluding the rust standard library
#![no_main] // disabling all Rust-level entry points

// retrieve panic information
// without implementing the default panic handler
use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

// the program entry point
#[no_mangle] // no name mangling - https://en.wikipedia.org/wiki/Name_mangling
pub extern "C" fn _start() -> ! {
    // The "!" return value is rust's special "never" return type
    // https://doc.rust-lang.org/nightly/std/primitive.never.html

    // named _start by default:

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
