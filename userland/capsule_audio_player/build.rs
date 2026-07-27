// Cross-compiles the vendored minimp3 decoder freestanding for the NONOS user
// target, mirroring the proven QuickJS recipe in userland/nonos_qjs/build.rs:
// no OS headers, no libc, only the shared shim, with memcpy/memset/memmove
// satisfied by compiler_builtins-mem.
use std::process::Command;

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
    b.flag("--target=x86_64-unknown-none");
    b.flag("-ffreestanding");
    b.flag("-fno-stack-protector");
    b.flag("-fno-builtin");
    b.flag("-mno-red-zone");
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
