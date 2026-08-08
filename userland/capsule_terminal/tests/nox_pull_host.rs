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

#[path = "../src/command/builtin/nox/pull/target.rs"]
pub mod target;

#[path = "../src/command/builtin/nox/pull/args.rs"]
pub mod args;

#[path = "../src/command/builtin/nox/pull/http.rs"]
pub mod http;

#[path = "../src/command/builtin/nox/pull/framing.rs"]
pub mod framing;

#[path = "../src/command/builtin/nox/pull/recurse.rs"]
pub mod recurse;

#[path = "../src/command/builtin/nox/pull/verify.rs"]
pub mod verify;

#[path = "../src/command/builtin/nox/pull/redirect.rs"]
pub mod redirect;

pub mod term {
    pub mod util {
        pub fn format_u64(mut n: u64, out: &mut [u8]) -> usize {
            if n == 0 {
                out[0] = b'0';
                return 1;
            }
            let mut tmp = [0u8; 24];
            let mut i = 0;
            while n > 0 {
                tmp[i] = b'0' + (n % 10) as u8;
                n /= 10;
                i += 1;
            }
            for j in 0..i {
                out[j] = tmp[i - 1 - j];
            }
            i
        }
    }
}

#[path = "../src/command/builtin/nox/pull/progress.rs"]
pub mod progress;

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
    assert_eq!(a.target.hostname, b"10.0.2.2");
    assert_eq!(a.target.port, 8000);
    assert_eq!(a.target.host, b"10.0.2.2:8000");
    assert_eq!(a.target.path, b"/report.pdf");
    assert_eq!(a.dest, b"/docs/report.pdf");
    assert!(!a.target.is_dir);
    assert!(!a.no_clobber);
}

#[test]
fn args_parse_dir_trailing_slash() {
    let a = ok_val(args::parse(&[&b"10.0.2.2:8000/photos/"[..], &b"/docs/photos/"[..]]));
    assert!(a.target.is_dir);
    assert_eq!(a.target.path, b"/photos/");
}

#[test]
fn args_reject_bad() {
    assert!(args::parse(&[]).is_err());
    assert!(args::parse(&[&b"a/x"[..], &b"/d"[..], &b"/e"[..]]).is_err());
    assert!(args::parse(&[&b"hostonly"[..]]).is_err());
    assert!(args::parse(&[&b"host:bad/x"[..]]).is_err());
    assert!(args::parse(&[&b"/x"[..]]).is_err());
}

#[test]
fn target_parse_ip_with_port() {
    let t = ok_val(target::parse_target(b"10.0.2.2:8000/report.pdf"));
    assert_eq!(t.hostname, b"10.0.2.2");
    assert_eq!(t.host, b"10.0.2.2:8000");
    assert_eq!(t.port, 8000);
    assert_eq!(t.path, b"/report.pdf");
    assert!(!t.is_dir);
}

#[test]
fn target_parse_hostname_default_port() {
    let t = ok_val(target::parse_target(b"example.com/a.txt"));
    assert_eq!(t.hostname, b"example.com");
    assert_eq!(t.host, b"example.com");
    assert_eq!(t.port, 80);
    assert_eq!(t.path, b"/a.txt");
}

#[test]
fn target_parse_dir_trailing_slash() {
    let t = ok_val(target::parse_target(b"host:9/photos/"));
    assert!(t.is_dir);
    assert_eq!(t.port, 9);
}

#[test]
fn target_reject_missing_path_and_bad_port() {
    assert!(target::parse_target(b"host:80").is_err());
    assert!(target::parse_target(b"host:99999/x").is_err());
    assert!(target::parse_target(b"host:/x").is_err());
}

#[test]
fn default_dest_file_and_dir() {
    let f = ok_val(target::parse_target(b"h/sub/report.pdf"));
    assert_eq!(target::default_dest(&f), b"report.pdf");
    let d = ok_val(target::parse_target(b"h/photos/"));
    assert_eq!(target::default_dest(&d), b"photos/");
}

#[test]
fn default_dest_host_root_uses_hostname() {
    let d = ok_val(target::parse_target(b"example.com/"));
    assert_eq!(target::default_dest(&d), b"example.com/");
    let f = ok_val(target::parse_target(b"10.0.2.2:8000/"));
    assert_eq!(target::default_dest(&f), b"10.0.2.2/");
}

#[test]
fn args_flag_and_default_dest() {
    let a = ok_val(args::parse(&[&b"-n"[..], &b"10.0.2.2:8000/x.txt"[..]]));
    assert!(a.no_clobber);
    assert_eq!(a.dest, b"x.txt");
    let b = ok_val(args::parse(&[&b"10.0.2.2:8000/x.txt"[..], &b"/docs/x"[..]]));
    assert!(!b.no_clobber);
    assert_eq!(b.dest, b"/docs/x");
}

#[test]
fn http_build_get_shape() {
    let r = http::build_get(b"10.0.2.2:8000", b"/a.txt", b"");
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

#[test]
fn progress_file_line_has_name_and_size() {
    let l = progress::file_line(b"/docs/report.pdf", 4096);
    assert_eq!(l, b"/docs/report.pdf  4096 bytes");
}

#[test]
fn progress_summary_counts() {
    let t = progress::Tally { ok: 3, skipped: 1, failed: 2 };
    assert_eq!(progress::summary(&t), b"3 ok, 1 skipped, 2 failed");
}

#[path = "../../inflate/src"]
mod inflate {
    #[path = "bits.rs"]
    pub mod bits;
    #[path = "codes.rs"]
    pub mod codes;
    #[path = "dynamic.rs"]
    pub mod dynamic;
    #[path = "fixed.rs"]
    pub mod fixed;
    #[path = "gzip.rs"]
    pub mod gzip;
    #[path = "huff.rs"]
    pub mod huff;
    #[path = "inflate_raw.rs"]
    pub mod inflate_raw;
    #[path = "stored.rs"]
    pub mod stored;
    #[path = "tables.rs"]
    pub mod tables;
    #[path = "zlib.rs"]
    pub mod zlib;
}

#[test]
fn gunzip_hello_roundtrips() {
    let gz: [u8; 25] = [
        0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
        0x07, 0x00, 0x86, 0xa6, 0x10, 0x36, 0x05, 0x00, 0x00, 0x00,
    ];
    let out = inflate::gzip::gunzip(&gz).expect("gunzip");
    assert_eq!(&out, b"hello");
}

#[test]
fn http_build_get_ka_is_keep_alive() {
    let r = http::build_get_ka(b"example.com", b"/f.txt", b"");
    assert!(scan::find(&r, b"Connection: keep-alive").is_some());
    assert!(scan::find(&r, b"Accept-Encoding: gzip").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn frame_splits_two_concatenated_bodies() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\nabc");
    buf.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\ndef");
    let f1 = framing::frame(&buf).unwrap().expect("first framed");
    assert_eq!(&buf[f1.body.clone()], b"abc");
    let n1 = f1.body.end;
    let f2 = framing::frame(&buf[n1..]).unwrap().expect("second framed");
    assert_eq!(&buf[n1..][f2.body.clone()], b"def");
}

#[test]
fn frame_returns_none_when_body_incomplete() {
    let partial = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nab";
    assert!(framing::frame(partial).unwrap().is_none());
}

#[test]
fn frame_rejects_missing_content_length_for_keepalive() {
    let no_cl = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n";
    match framing::frame(no_cl) {
        Err((retryable, _)) => assert!(retryable),
        Ok(_) => panic!("expected keep-alive framing rejection"),
    }
}

#[test]
fn http_build_head_is_head_without_encoding() {
    let r = http::build_head(b"example.com", b"/f.txt", b"");
    assert!(r.starts_with(b"HEAD /f.txt HTTP/1.1"));
    assert!(scan::find(&r, b"Connection: keep-alive").is_some());
    assert!(scan::find(&r, b"Accept-Encoding").is_none());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn verify_sidecar_path_appends_suffix() {
    assert_eq!(verify::sidecar_path(b"/dir/f.txt"), b"/dir/f.txt.sha256".to_vec());
}

#[test]
fn verify_parses_lowercase_digest() {
    let hex = b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    let d = verify::parse_sha256_hex(hex).expect("parsed");
    assert_eq!(d[0], 0xe3);
    assert_eq!(d[31], 0x55);
}

#[test]
fn verify_parses_sha256sum_output_with_filename() {
    let line = b"  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  file.bin\n";
    let d = verify::parse_sha256_hex(line).expect("parsed");
    assert_eq!(d[0], 0xe3);
}

#[test]
fn verify_accepts_uppercase_hex() {
    let lo = verify::parse_sha256_hex(
        b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
    let hi = verify::parse_sha256_hex(
        b"E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
    );
    assert_eq!(lo, hi);
}

#[test]
fn verify_rejects_short_digest() {
    assert!(verify::parse_sha256_hex(b"deadbeef").is_none());
}

#[test]
fn verify_rejects_non_hex_char() {
    let bad = b"g3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    assert!(verify::parse_sha256_hex(bad).is_none());
}

#[test]
fn redirect_absolute_path_replaces_whole_path() {
    assert_eq!(redirect::resolve(b"h", b"/a/b", b" /new").unwrap(), b"/new".to_vec());
}

#[test]
fn redirect_relative_resolves_against_cur_dir() {
    assert_eq!(
        redirect::resolve(b"h", b"/dir/file.txt", b"next.txt").unwrap(),
        b"/dir/next.txt".to_vec()
    );
}

#[test]
fn redirect_absolute_url_same_host_extracts_path() {
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host/x").unwrap(), b"/x".to_vec());
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host:8080/x").unwrap(), b"/x".to_vec());
    assert_eq!(redirect::resolve(b"host", b"/a", b"HTTP://host/x").unwrap(), b"/x".to_vec());
}

#[test]
fn redirect_absolute_url_no_path_is_root() {
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host").unwrap(), b"/".to_vec());
}

#[test]
fn redirect_refuses_cross_host() {
    assert!(redirect::resolve(b"host", b"/a", b"http://other/x").is_err());
}

#[test]
fn redirect_refuses_https_downgrade_guard() {
    assert!(redirect::resolve(b"host", b"/a", b"https://host/x").is_err());
}

#[test]
fn redirect_rejects_empty_location() {
    assert!(redirect::resolve(b"host", b"/a", b"   ").is_err());
}

#[test]
fn http_build_put_head_has_content_length() {
    let r = http::build_put_head(b"h.local", b"/up/x.bin", 42, b"");
    assert!(r.starts_with(b"PUT /up/x.bin HTTP/1.1"));
    assert!(scan::find(&r, b"Host: h.local").is_some());
    assert!(scan::find(&r, b"Content-Length: 42").is_some());
    assert!(scan::find(&r, b"Accept-Encoding").is_none());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_put_head_includes_extra_headers() {
    let r = http::build_put_head(b"h", b"/p", 0, b"Authorization: Basic abc\r\n");
    assert!(scan::find(&r, b"Authorization: Basic abc").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn args_parse_skip_unchanged_flag() {
    let argv: [&[u8]; 2] = [b"-u", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert!(a.skip_unchanged);
    assert!(!a.no_clobber);
}

#[test]
fn args_parse_both_flags_any_order() {
    let argv: [&[u8]; 3] = [b"-u", b"-n", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert!(a.skip_unchanged);
    assert!(a.no_clobber);
}

#[test]
fn args_parse_auth_and_header() {
    let argv: [&[u8]; 5] = [b"-a", b"user:pass", b"-H", b"X-Test: 1", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert_eq!(a.auth.as_deref(), Some(&b"user:pass"[..]));
    assert_eq!(a.headers.len(), 1);
    assert_eq!(&a.headers[0], b"X-Test: 1");
}

#[test]
fn args_reject_crlf_in_header() {
    let argv: [&[u8]; 3] = [b"-H", b"X: 1\r\nEvil: 2", b"example.com/f.txt"];
    assert!(args::parse(&argv).is_err());
}

#[test]
fn http_build_get_includes_extra_headers() {
    let r = http::build_get(b"h", b"/p", b"Authorization: Basic abc\r\n");
    assert!(scan::find(&r, b"Authorization: Basic abc").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[path = "../../base64/src"]
mod base64 {
    #[path = "decode.rs"]
    pub mod decode;
    #[path = "encode.rs"]
    pub mod encode;
}

#[test]
fn base64_encode_basic_auth_credential() {
    assert_eq!(&base64::encode::encode(b"user:pass"), b"dXNlcjpwYXNz");
}
