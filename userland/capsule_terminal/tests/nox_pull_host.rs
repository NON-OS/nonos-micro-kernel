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

fn ok_val<T, E>(r: Result<T, E>) -> T {
    match r {
        Ok(v) => v,
        Err(_) => std::process::exit(1),
    }
}

fn some_val<T>(o: Option<T>) -> T {
    match o {
        Some(v) => v,
        None => std::process::exit(1),
    }
}

#[path = "../src/command/builtin/nox/pull/scan.rs"]
pub mod scan;

#[path = "../src/command/builtin/nox/pull/ipv4.rs"]
pub mod ipv4;

#[path = "../src/command/builtin/nox/pull/args.rs"]
pub mod args;

#[path = "../src/command/builtin/nox/pull/http.rs"]
pub mod http;

#[path = "../src/command/builtin/nox/pull/recurse.rs"]
pub mod recurse;

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

#[test]
fn args_parse_file() {
    let a = ok_val(args::parse(&[&b"10.0.2.2:8000/report.pdf"[..], &b"/docs/report.pdf"[..]]));
    assert_eq!(a.ip, [10, 0, 2, 2]);
    assert_eq!(a.port, 8000);
    assert_eq!(a.host, b"10.0.2.2:8000");
    assert_eq!(a.path, b"/report.pdf");
    assert_eq!(a.dest, b"/docs/report.pdf");
    assert!(!a.is_dir);
}

#[test]
fn args_parse_dir_trailing_slash() {
    let a = ok_val(args::parse(&[&b"10.0.2.2:8000/photos/"[..], &b"/docs/photos/"[..]]));
    assert!(a.is_dir);
    assert_eq!(a.path, b"/photos/");
}

#[test]
fn args_reject_bad() {
    assert!(args::parse(&[&b"10.0.2.2:8000/x"[..]]).is_err());
    assert!(args::parse(&[&b"999.0.0.1:80/x"[..], &b"/d"[..]]).is_err());
    assert!(args::parse(&[&b"10.0.2.2/x"[..], &b"/d"[..]]).is_err());
}

#[test]
fn http_build_get_shape() {
    let r = http::build_get(b"10.0.2.2:8000", b"/a.txt");
    assert!(scan::find(&r, b"GET /a.txt HTTP/1.1\r\n").is_some());
    assert!(scan::find(&r, b"Host: 10.0.2.2:8000\r\n").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_parse_content_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nBODY";
    let p = some_val(http::parse_head(raw));
    assert_eq!(p.status, 200);
    assert_eq!(p.content_length, Some(4));
    assert!(!p.chunked);
    assert_eq!(&raw[p.body_off..], b"BODY");
}

#[test]
fn http_parse_404_and_chunked() {
    let p = some_val(http::parse_head(b"HTTP/1.1 404 Not Found\r\n\r\n"));
    assert_eq!(p.status, 404);
    let c = some_val(http::parse_head(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n"));
    assert!(c.chunked);
}

#[test]
fn autoindex_extracts_files_and_dirs() {
    let html = br#"<ul><li><a href="a.png">a.png</a></li>
<li><a href="sub/">sub/</a></li></ul>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 2);
    assert_eq!(e[0].name, b"a.png");
    assert!(!e[0].is_dir);
    assert_eq!(e[1].name, b"sub/");
    assert!(e[1].is_dir);
}

#[test]
fn autoindex_skips_parent_and_absolute() {
    let html = br#"<a href="../">up</a><a href="/root">r</a><a href="ok.txt">ok</a>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].name, b"ok.txt");
}

#[test]
fn autoindex_rejects_traversal_names() {
    let html = br#"<a href="a/../../etc">x</a><a href="s/../../y">y</a><a href="..\z">z</a><a href="good.txt">g</a>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].name, b"good.txt");
}
