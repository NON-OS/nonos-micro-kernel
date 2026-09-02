// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
// Host-side harness for the block's context line and the identity behind it.
// Compiled with the host toolchain:
//   rustc --edition 2021 --crate-type=rlib --crate-name nonos_policy_proto \
//     ../policy_proto/src/lib.rs -o /tmp/libnonos_policy_proto.rlib
//   rustc --edition 2021 --test tests/context_host.rs \
//     --extern nonos_policy_proto=/tmp/libnonos_policy_proto.rlib -o /tmp/context_host \
//     && /tmp/context_host
//
// `wire.rs` is written against the real wire crate rather than a mirror of it,
// because a mirror would pass while the ABI drifted underneath it, so that
// crate is built first and handed over as an extern.
//
// The IPC and the cache need a kernel, so neither is pulled in. What is under
// test is everything that decides what gets drawn: the wire framing, the
// sanitizing of a reply, the fallback choice, and the formatting.

#[path = "../src/term"]
mod term {
    pub mod cwd {
        mod home;
        mod shorten;
        mod strip_home;
        pub use home::HOME;
        pub use shorten::shorten;
        pub use strip_home::strip_home;
    }

    pub mod util {
        mod copy_into;
        pub use copy_into::copy_into;
    }

    pub mod identity {
        mod choose;
        mod sanitize;
        mod wire;
        pub use choose::{choose, HOST_FALLBACK, USER};
        pub use sanitize::hostname_len;
        pub use wire::{decode_str, request, REQ_LEN};
    }

    pub mod context;
}

use nonos_policy_proto::{Header, E_OK, HDR_LEN, KIND_STR, OP_GET};
use term::context::context_line;
use term::cwd::{shorten, strip_home, HOME};
use term::identity::{choose, decode_str, hostname_len, request, HOST_FALLBACK, REQ_LEN, USER};

const HOSTNAME: u32 = 0x0301;

fn line(host: &[u8], cwd: &[u8], home: &[u8]) -> String {
    let mut out = [0u8; 96];
    let n = context_line(USER, host, cwd, home, &mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}

fn reply(op: u16, field: u32, kind: u8, status: u16, body: &[u8]) -> Vec<u8> {
    let mut v = vec![0u8; HDR_LEN];
    let hdr = Header { op, field, kind, status, payload_len: body.len() as u16 };
    hdr.encode(&mut v[..HDR_LEN]);
    v.extend_from_slice(body);
    v
}

#[test]
fn the_context_line_reads_as_the_mockup_does() {
    assert_eq!(line(b"station", b"/workspace", b"/"), "nonos@station:/workspace");
    assert_eq!(line(b"station", b"/home/n/workspace", b"/home/n"), "nonos@station:~/workspace");
}

#[test]
fn the_home_directory_itself_is_a_bare_tilde() {
    assert_eq!(line(b"h", b"/home/n", b"/home/n"), "nonos@h:~");
}

#[test]
fn a_path_that_merely_starts_with_home_is_not_shortened() {
    assert_eq!(line(b"h", b"/homework", b"/home"), "nonos@h:/homework");
    assert_eq!(strip_home(b"/homework", b"/home"), None);
}

#[test]
fn root_and_an_unset_home_shorten_nothing() {
    assert_eq!(line(b"h", b"/a", b"/"), "nonos@h:/a");
    assert_eq!(line(b"h", b"/a", b""), "nonos@h:/a");
    assert_eq!(strip_home(b"/a", b""), None);
    assert_eq!(strip_home(b"/a", b"/"), None);
}

#[test]
fn a_line_longer_than_the_grid_is_cut_rather_than_overrunning() {
    let cwd = vec![b'x'; 200];
    let mut out = [0u8; 96];
    let n = context_line(USER, b"station", &cwd, b"", &mut out);
    assert_eq!(n, 96);
    assert!(out.starts_with(b"nonos@station:xxx"));
}

#[test]
fn an_unreachable_policy_falls_back_without_claiming_a_configured_name() {
    assert_eq!(choose(b""), HOST_FALLBACK);
    assert_eq!(HOST_FALLBACK, b"nonos");
    assert_eq!(choose(b"station"), b"station");
}

#[test]
fn a_configured_hostname_survives_its_fixed_width_padding() {
    let mut padded = b"station".to_vec();
    padded.resize(64, 0);
    assert_eq!(hostname_len(&padded), 7);
    assert_eq!(hostname_len(b"nonos-1.local"), 13);
    assert_eq!(hostname_len(b""), 0);
}

#[test]
fn a_hostname_carrying_anything_a_hostname_cannot_is_cut_there() {
    assert_eq!(hostname_len(b"st ation"), 2);
    assert_eq!(hostname_len(b"st\nation"), 2);
    assert_eq!(hostname_len(b"\x1b[31m"), 0);
}

#[test]
fn the_request_is_a_bare_header_naming_the_field() {
    let mut tx = [0u8; REQ_LEN];
    request(HOSTNAME, &mut tx);
    let h = Header::decode(&tx).unwrap();
    assert_eq!((h.op, h.field, h.kind, h.payload_len), (OP_GET, HOSTNAME, KIND_STR, 0));
}

#[test]
fn a_matching_reply_yields_the_string_after_the_header() {
    let rx = reply(OP_GET, HOSTNAME, KIND_STR, E_OK, b"station");
    assert_eq!(decode_str(HOSTNAME, &rx), Some(&b"station"[..]));
}

#[test]
fn a_reply_that_answers_a_different_question_is_refused() {
    let ok = E_OK;
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, 0x0302, KIND_STR, ok, b"x")), None);
    assert_eq!(decode_str(HOSTNAME, &reply(9, HOSTNAME, KIND_STR, ok, b"x")), None);
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, HOSTNAME, 1, ok, b"x")), None);
}

#[test]
fn an_error_reply_and_a_truncated_one_yield_nothing() {
    assert_eq!(decode_str(HOSTNAME, &reply(OP_GET, HOSTNAME, KIND_STR, 91, b"")), None);
    let mut short = reply(OP_GET, HOSTNAME, KIND_STR, E_OK, b"station");
    short.truncate(HDR_LEN + 3);
    assert_eq!(decode_str(HOSTNAME, &short), None);
    assert_eq!(decode_str(HOSTNAME, &[0u8; 4]), None);
}

#[test]
fn home_is_the_users_directory() {
    assert!(HOME.ends_with(USER), "HOME must end in the identity's user");
    assert_eq!(HOME.first(), Some(&b'/'));
    assert_ne!(HOME.last(), Some(&b'/'));
}

#[test]
fn the_prompt_shows_a_bare_tilde_in_the_home_itself() {
    let mut out = [0u8; 128];
    let n = context_line(USER, b"station", HOME, HOME, &mut out);
    assert_eq!(&out[..n], b"nonos@station:~");
}

#[test]
fn the_prompt_shows_the_folder_under_the_tilde() {
    let mut cwd = Vec::from(HOME);
    cwd.extend_from_slice(b"/workspace");
    let mut out = [0u8; 128];
    let n = context_line(USER, b"station", &cwd, HOME, &mut out);
    assert_eq!(&out[..n], b"nonos@station:~/workspace");
}

#[test]
fn shorten_agrees_with_the_block_header() {
    let mut b = [0u8; 160];
    assert_eq!(shorten(HOME, HOME, &mut b), b"~");
    let mut c = Vec::from(HOME);
    c.extend_from_slice(b"/workspace");
    let mut b2 = [0u8; 160];
    assert_eq!(shorten(&c, HOME, &mut b2), b"~/workspace");
}

#[test]
fn shorten_leaves_a_path_outside_the_home_alone() {
    let mut b = [0u8; 160];
    assert_eq!(shorten(b"/etc/keys", HOME, &mut b), b"/etc/keys");
}
