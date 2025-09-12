# MinSL 
*A minimal functional shell that can also be booted as Linux init using an initramfs*


## Overview
MinSL (**Minimal Shell for Linux**) is a tiny Unix-like shell implemented in Rust with **no std and no libc**.  
It communicates with the Linux kernel entirely through **raw syscalls** and a small amount of hand-written assembly.

This project started as a learning exercise in **systems programming** and grew into a functional shell you can boot inside a Linux kernel using an initramfs.


## Features
- Hand-rolled entrypoint (`_start`) and syscall stubs in x86-64 assembly
- Syscalls implemented directly: `write`, `read`, `fork`, `execve`, `waitid`, `exit`
- Custom minimal libc replacements: `memcpy`, `memset`, `memcmp`, `memmove`
- Custom minimal cstring manipulation replacements: `strlen`, `strcmp`, `strncmp`, `strchr`
- Supports launching external programs (e.g. `ls`, `whoami`, `mkdir`, `touch` …)
- Tokenizes the input string and passes arguments to execve
- Can run as Linux `init` inside an **initramfs**
- Ultra small footprint: **~1.8 KB** statically linked ELF binary


## What I Learned
- Rust `no_std` and `no_main` programming
- System V AMD64 ABI (function arguments, return values, register conventions)
- Linux syscalls and process management (`fork`, `execve`, `waitid`)
- Binary size optimization with linker flags, LTO, and symbol stripping
- Writing essential libc functions by hand
- Implementing a tokenizer using raw null terminated strings


## Building
You’ll need a nightly Rust toolchain for compilation. The project contains a bash script to abstract away the build process.

```bash
# You may need to make build.sh executable
chmod +x build.sh
# See build options
./build.sh -h
# Build release binary (with no_std + LTO)
./build.sh -r
```


### Build Dependencies

To build MinSL you’ll need:

- **Rust nightly toolchain** (required for `#![no_std]` + `-Z build-std`)  
  Install with:
  ```bash
  # Install Rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Install the nightly toolchain
  rustup install nightly
  ```

- **Cargo** (comes with Rust)

- **Assembler and linker**  
  - On Linux: `binutils` (provides `as`, `ld`, etc.)  
  - On Fedora:  
    ```bash
    sudo dnf install binutils gcc
    ```
  - On Debian/Ubuntu:  
    ```bash
    sudo apt install build-essential
    ```
  - On Arch Linux:  
    ```bash
    sudo pacman -S base-devel
    ```


## Running
Run from the project root (requires no external libraries, build produces a statically linked ELF binary).

```bash
# Release build
./target/x86_64-unknown-linux-gnu/release/minsl
# Debug build
./target/x86_64-unknown-linux-gnu/debug/minsl
```


## Project Structure
```
.
├── build.rs
├── build.sh
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── asm
    │   ├── entrypoint.S
    │   └── syscalls.S
    ├── cstr.rs
    ├── lexer.rs
    ├── main.rs
    ├── memlibc.rs
    ├── siglibc.rs
    └── syscalls.rs
```


## Disclaimer
This is a **learning project**, not a production-ready shell.  
It’s intentionally minimal, rough around the edges, and built to explore how much you can do with Rust and raw syscalls.


## Future Ideas
- [x] Support for command arguments parsing without libc (e.g. `mkdir testdir`)
- [ ] `$PATH` lookup so you can run commands without absolute paths
- [ ] Simple builtins (`cd`, `exit`)
- [ ] Error reporting on failed `execve`
- [ ] Support piping and redirection


## License
Licensed under the MIT License. See LICENSE for details.


## Acknowledgments
This project was inspired by [this video](https://youtu.be/u2Juz5sQyYQ?si=7KzM8g54TOa82CNC) by [Nir Lichtman](https://www.youtube.com/@nirlichtman) and the accompanying [GitHub repository](https://github.com/nir9/welcome/tree/master/lnx/very-minimal-shell). Both provided background and concepts on writing a shell, which I reimplemented in Rust.
