// Cross-compiles the vendored QuickJS-ng core freestanding for the NONOS user
// target. The recipe matches the proven bare x86_64 build: no OS headers, no
// libc, only the minimal shim, with the symbols it references satisfied by the
// Rust stub layer in src/stubs.rs.
use std::process::Command;

fn target_arch() -> String {
    std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH")
}

fn clang_resource_include() -> String {
    let out = Command::new("clang")
        .arg("-print-resource-dir")
        .output()
        .expect("clang -print-resource-dir");
    let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
    format!("{dir}/include")
}

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // hosted: native build for the render harness, hosted libc instead of the
    // freestanding shim and stub layers.
    if std::env::var("CARGO_FEATURE_HOSTED").is_ok() {
        let mut b = cc::Build::new();
        b.compiler("clang");
        b.warnings(false);
        b.flag("-O2");
        b.flag("-DNDEBUG");
        // macOS spells malloc_usable_size differently.
        if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
            b.flag("-Dmalloc_usable_size=malloc_size");
        }
        b.flag(&format!("-I{root}/vendor"));
        for f in
            ["quickjs.c", "libregexp.c", "libunicode.c", "dtoa.c", "eval_shim.c", "dom_bindings.c"]
        {
            b.file(format!("{root}/vendor/{f}"));
            println!("cargo:rerun-if-changed=vendor/{f}");
        }
        b.compile("qjs");
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }
    let mut b = cc::Build::new();
    b.compiler("clang");
    b.no_default_flags(true);
    b.warnings(false);
    // The capsule triple is a JSON target clang has never heard of, so name the
    // bare triple for the same architecture instead. Getting this wrong is not
    // a compile error: clang builds for whatever it defaulted to and the
    // mismatch only surfaces as "incompatible" objects at link time.
    let arch = target_arch();
    b.flag(&format!("--target={arch}-unknown-none"));
    b.flag("-ffreestanding");
    b.flag("-fno-stack-protector");
    b.flag("-fno-builtin");
    // The red zone is an x86_64 ABI rule and the flag is rejected elsewhere.
    if arch == "x86_64" {
        b.flag("-mno-red-zone");
    }
    // The browser capsule links as a position-independent executable, so the C
    // objects must use PIC relocations to link against the Rust PIC code.
    b.flag("-fPIC");
    b.flag("-O2");
    b.flag("-DNDEBUG");
    b.flag("-Dalloca=__builtin_alloca");
    b.flag("-nostdinc");
    b.flag(&format!("-isystem{root}/shim"));
    b.flag(&format!("-isystem{}", clang_resource_include()));
    b.flag(&format!("-I{root}/vendor"));
    for f in [
        "quickjs.c",
        "libregexp.c",
        "libunicode.c",
        "dtoa.c",
        "stubs.c",
        "eval_shim.c",
        "dom_bindings.c",
    ] {
        b.file(format!("{root}/vendor/{f}"));
        println!("cargo:rerun-if-changed=vendor/{f}");
    }
    b.compile("qjs");
    println!("cargo:rerun-if-changed=build.rs");
}
