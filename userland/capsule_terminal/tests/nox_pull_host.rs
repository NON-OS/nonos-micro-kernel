// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side unit harness for the pure nox-pull parsers. Compiled with the host
// toolchain (not the capsule target) so the layout-independent parsing logic is
// exercised without QEMU:
//   rustc --edition 2021 --test tests/nox_pull_host.rs -o /tmp/nox_pull_host && /tmp/nox_pull_host
//
// Modules are declared flat at the crate root so `super::scan` inside http.rs /
// recurse.rs resolves to this root (mirroring `pull::scan` in the capsule), and
// so #[path] stays relative to the real tests/ directory.

extern crate alloc;

#[path = "../src/command/builtin/nox/pull/scan.rs"]
pub mod scan;

#[test]
fn find_locates_crlf_gap() {
    let b = b"HTTP/1.1 200 OK\r\nX: y\r\n\r\nBODY";
    assert_eq!(scan::find(b, b"\r\n\r\n"), Some(21));
}

#[test]
fn split_lines_strips_cr() {
    let lines = scan::split_lines(b"a\r\nbb\r\nccc");
    assert_eq!(lines, vec![&b"a"[..], &b"bb"[..], &b"ccc"[..]]);
}

#[test]
fn parse_usize_rejects_nondigits() {
    assert_eq!(scan::parse_usize(b" 4096 "), Some(4096));
    assert_eq!(scan::parse_usize(b"12x"), None);
}

#[test]
fn eq_ci_is_case_insensitive() {
    assert!(scan::eq_ci(b"Content-Length", b"content-length"));
    assert!(!scan::eq_ci(b"a", b"ab"));
}
