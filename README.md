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
### Individual OS alternative
- Linux
- - `cargo rustc -- -C link-arg=-nostartfiles`
- Windows
- - `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
- macOS
- - `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`

### Getting to run a QEMU instance of it (x86_64)
- I highly recommend using rustup for the ease of setting up.
- - `rusup default nightly`
- - `cargo install bootimage`
- - `rustup component add llvm-tools-preview`
- - `cargo bootimage`
- - NOTE: your terminal should be opened at the root directory at the repo.
- - If it isn't, then you need to modify the `file=` part to your full target path.
- - `qemu-system-x86_64 -drive format=raw, file=target/x86_64-config/debug/bootimage-operating-system-rust.bin`



## Other useful materials on this topic
- https://gist.github.com/cb372/5f6bf16ca0682541260ae52fc11ea3bb
- https://docs.rust-embedded.org/book/
