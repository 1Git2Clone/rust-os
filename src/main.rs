#![no_std] // excluding the rust standard library
#![no_main] // disabling all Rust-level entry points

// retrieve panic information
// without implementing the default panic handler
use core::panic::PanicInfo;

// the program entry point
#[no_mangle] // no name mangling - https://en.wikipedia.org/wiki/Name_mangling
pub extern "C" fn _start() -> ! {
    // The "!" return value is rust's special "never" return type
    // https://doc.rust-lang.org/nightly/std/primitive.never.html

    // named _start by default:
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
