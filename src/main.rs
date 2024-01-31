// src/main.rs

#![no_std] // excluding the rust standard library
#![no_main] // disabling all Rust-level entry points

mod vga_buffer;

// retrieve panic information
// without implementing the default panic handler
use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// the program entry point
#[no_mangle] // no name mangling - https://en.wikipedia.org/wiki/Name_mangling
pub extern "C" fn _start() -> ! {
    // The "!" return value is rust's special "never" return type
    // https://doc.rust-lang.org/nightly/std/primitive.never.html

    println!("Hello World{}", "!");
    println!(
        "Continuating line of the last line!\nSome numbers: {}, {}",
        69, 420
    );
    for n in 0..100 {
        println!("{}. Ligma bools...", n);
    }

    // panic test.
    // panic!("Some panic message");
    loop {}
}
