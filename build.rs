fn main() {
    println!("cargo:rerun-if-changed=src/asm/syscalls.S");
    println!("cargo:rerun-if-changed=src/asm/entrypoint.S");

    cc::Build::new()
        .file("src/asm/syscalls.S")
        .compile("syscalls");

    cc::Build::new()
        .file("src/asm/entrypoint.S")
        .compile("entrypoint");
}
