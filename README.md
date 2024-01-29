# Rust built operating system

## Details
- It's based on this wiki: https://os.phil-opp.com/freestanding-rust-binary/
- The primary goal for this is educating myself on how to operating systems work.
- The program is built with --edition 2021 using rustup. My work environment:
- - Arch Linux on x86_64 AMD Ryzen 5 3600X CPU

## Building the app
In order to actually build the application,you need to define
a special target value. To avoid any linker errors you need to
disable the dependency for the default C runtime.

`rustup target add thumbv7em-none-eabihf`

`cargo build --target thumbv7em-none-eabihf`

- Reference: [Clang LLVM](https://clang.llvm.org/docs/CrossCompilation.html#target-triple)

## Other useful materials on this topic
- https://gist.github.com/cb372/5f6bf16ca0682541260ae52fc11ea3bb
- https://docs.rust-embedded.org/book/
