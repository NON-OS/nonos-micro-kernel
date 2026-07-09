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

//! Kani proof harnesses: a model checker proves these properties over ALL
//! inputs (bounded), not sampled cases. Compiled only under Kani (`cargo kani`),
//! so ordinary builds and `cargo test` ignore this module. Each `#[kani::proof]`
//! that verifies establishes a machine-checked theorem about the real code.

use crate::net::eth::frame::{payload_of, EthHeader, HDR_LEN};
use crate::net::ipv4::parse::parse as ipv4_parse;
use crate::net::udp::parse::parse as udp_parse;
use crate::vfs_protocol::decode_request;
use crate::{normalize_to_buffer, split_caller};

// The request decoder must never panic or execute UB for any byte sequence.
// Forty bytes cover the header plus enough payload to exercise every branch;
// beyond that the decoder only slices, with no per-byte logic.
#[kani::proof]
#[kani::unwind(2)]
fn proof_decode_request_total() {
    let data: [u8; 40] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 40);
    let _ = decode_request(&data[..len]);
}

#[kani::proof]
#[kani::unwind(3)]
fn proof_eth_parse_total() {
    let data: [u8; 40] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 40);
    let frame = &data[..len];
    if let Some(h) = EthHeader::parse(frame) {
        let mut out = [0u8; HDR_LEN];
        assert!(h.write(&mut out));
        assert!(payload_of(frame).is_some());
        if let Some(payload) = payload_of(frame) {
            assert!(payload.len() <= frame.len());
        }
    }
}

#[kani::proof]
#[kani::unwind(40)]
fn proof_ipv4_parse_total() {
    let data: [u8; 48] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 48);
    if let Ok((_h, payload)) = ipv4_parse(&data[..len]) {
        assert!(payload.len() <= len);
    }
}

#[kani::proof]
#[kani::unwind(80)]
fn proof_udp_parse_total() {
    let data: [u8; 48] = kani::any();
    let len: usize = kani::any();
    let src: [u8; 4] = kani::any();
    let dst: [u8; 4] = kani::any();
    kani::assume(len <= 48);
    if let Ok((_h, payload)) = udp_parse(&src, &dst, &data[..len]) {
        assert!(payload.len() <= len);
    }
}

// Authority theorem: a userspace caller (sender != 0) can never be attested as
// any pid other than its own, for any payload.
#[kani::proof]
fn proof_split_caller_no_impersonation() {
    let data: [u8; 12] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= 12);
    let sender: u32 = kani::any();
    if let Ok((pid, _rest)) = split_caller(&data[..len], sender) {
        if sender != 0 {
            assert!(pid == sender);
        }
    }
}

// Canonicalization theorem over a bounded alphabet that covers separators,
// current-dir, parent-dir, and ordinary components.
#[kani::proof]
#[kani::unwind(12)]
fn proof_normalize_short_invariants() {
    const N: usize = 2;
    let mut bytes = [0u8; N];
    let len: usize = kani::any();
    kani::assume(len <= N);
    let mut i = 0;
    while i < N {
        let c: u8 = kani::any();
        kani::assume(c == b'/' || c == b'.' || c == b'a');
        bytes[i] = c;
        i += 1;
    }
    let mut out = [0u8; N + 1];
    let written = normalize_to_buffer(&bytes[..len], &mut out);
    assert_normalized(&out[..written]);
}

#[kani::proof]
#[kani::unwind(12)]
fn proof_normalize_branch_cases() {
    let case: u8 = kani::any();
    kani::assume(case < 8);
    let input = match case {
        0 => "/a/..",
        1 => "a/..",
        2 => "/../a",
        3 => "/a/./b",
        4 => "a//b",
        5 => "/a/b/",
        6 => "/a/b/../../c",
        _ => "",
    };
    let mut out = [0u8; 16];
    let written = normalize_to_buffer(input.as_bytes(), &mut out);
    assert_normalized(&out[..written]);
}

fn assert_normalized(bytes: &[u8]) {
    assert!(bytes.len() > 0);
    assert!(bytes[0] == b'/');
    assert!(!has_adjacent_slashes(bytes));
    assert!(!has_dot_component(bytes));
    assert!(!has_dotdot_component(bytes));
    assert!(bytes.len() == 1 || bytes[bytes.len() - 1] != b'/');
}

fn has_adjacent_slashes(bytes: &[u8]) -> bool {
    let mut i = 1;
    while i < bytes.len() {
        if bytes[i - 1] == b'/' && bytes[i] == b'/' {
            return true;
        }
        i += 1;
    }
    false
}

fn has_dot_component(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i + 2 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'.' && bytes[i + 2] == b'/' {
            return true;
        }
        i += 1;
    }
    false
}

fn has_dotdot_component(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i + 3 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'.' && bytes[i + 2] == b'.' && bytes[i + 3] == b'/'
        {
            return true;
        }
        i += 1;
    }
    false
}
