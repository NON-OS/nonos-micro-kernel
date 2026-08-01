// Cross-compiles the vendored minimp3 decoder freestanding for the NONOS user
// target, mirroring the proven QuickJS recipe in userland/nonos_qjs/build.rs:
// no OS headers, no libc, only the shared shim, with memcpy/memset/memmove
// satisfied by compiler_builtins-mem.
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
    let vendor = format!("{root}/../../third_party/minimp3");
    let shim = format!("{root}/../nonos_qjs/shim");
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
    b.flag("-fPIC");
    b.flag("-O2");
    b.flag("-DNDEBUG");
    b.flag("-nostdinc");
    b.flag(&format!("-isystem{shim}"));
    b.flag(&format!("-isystem{}", clang_resource_include()));
    b.flag(&format!("-I{vendor}"));
    b.file(format!("{vendor}/minimp3.c"));
    b.compile("minimp3");
    println!("cargo:rerun-if-changed={vendor}/minimp3.c");
    println!("cargo:rerun-if-changed={vendor}/minimp3.h");
    println!("cargo:rerun-if-changed=build.rs");
}
