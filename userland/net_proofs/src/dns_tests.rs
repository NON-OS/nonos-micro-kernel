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

use crate::dns::{first_address, HDR_LEN};

extern crate alloc;
use alloc::vec::Vec;

// DNS responses come straight off the network. Parsing them must never panic
// and, critically, must never loop on a compression pointer (a classic
// denial-of-service in naive DNS parsers).

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn message(len: usize, seed: u32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..len).map(|_| (xorshift(&mut s) & 0xff) as u8).collect()
}

#[test]
fn first_address_never_panics_over_adversarial_messages() {
    for &len in &[0usize, 1, 2, 11, HDR_LEN, HDR_LEN + 1, 20, 30, 50, 100, 200, 512] {
        for seed in 0..20_000u32 {
            // Termination is part of the property: a hang here fails via timeout.
            let _ = first_address(&message(len, seed));
        }
    }
}

#[test]
fn compression_pointers_terminate_and_do_not_loop() {
    // A header claiming one question and one answer, followed by names that are
    // compression pointers, including one that points at itself. A naive parser
    // that followed pointers would loop forever; this must return.
    let mut m = vec![0u8; HDR_LEN];
    m[5] = 1; // qdcount = 1
    m[7] = 1; // ancount = 1
    m.push(0xC0);
    m.push(HDR_LEN as u8); // question name: pointer into the packet
    m.extend_from_slice(&[0, 1, 0, 1]); // qtype = A, qclass = IN
    m.push(0xC0);
    m.push(HDR_LEN as u8); // answer name: pointer
    let _ = first_address(&m);

    // A pointer whose target is the pointer itself.
    let mut loopy = vec![0u8; HDR_LEN];
    loopy[5] = 1;
    loopy[7] = 1;
    loopy.push(0xC0);
    loopy.push(HDR_LEN as u8);
    let _ = first_address(&loopy);

    // Every two-byte pointer value against a minimal packet.
    for hi in 0xC0u8..=0xFF {
        for lo in 0u8..=255 {
            let mut p = vec![0u8; HDR_LEN];
            p[5] = 1;
            p[7] = 1;
            p.push(hi);
            p.push(lo);
            let _ = first_address(&p);
        }
    }
}

// Ground-truth: a real, RFC 1035 wire-format response must parse to the CORRECT
// address, not merely "not panic". These pin the extractor to real bytes the way
// the fuzz tests above cannot: a wrong offset or byte order fails here.

#[test]
fn resolves_a_real_a_record_to_the_correct_ip() {
    // Response to a query for example.com A: answer 93.184.216.34, TTL 300, with
    // the answer name a compression pointer to the question (0xC0 0x0C).
    let m = alloc::vec![
        0x12, 0x34, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 0x03, b'c', b'o', b'm', 0x00,
        0x00, 0x01, 0x00, 0x01,
        0xC0, 0x0C, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2C, 0x00, 0x04,
        93, 184, 216, 34,
    ];
    let Ok((hdr, answer)) = first_address(&m) else { panic!("a well-formed response parses") };
    assert!(hdr.is_response());
    let a = answer.expect("an A answer is present");
    assert_eq!(a.ipv4, Some([93, 184, 216, 34]), "the exact address, right byte order");
    assert_eq!(a.ipv6, None);
    assert_eq!(a.ttl, 300, "TTL read big-endian from the right offset");
}

#[test]
fn resolves_a_real_aaaa_record_to_the_correct_ip() {
    // AAAA for 2606:2800:220:1:248:1893:25c8:1946.
    let m = alloc::vec![
        0x12, 0x34, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 0x03, b'c', b'o', b'm', 0x00,
        0x00, 0x1C, 0x00, 0x01,
        0xC0, 0x0C, 0x00, 0x1C, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2C, 0x00, 0x10,
        0x26, 0x06, 0x28, 0x00, 0x02, 0x20, 0x00, 0x01,
        0x02, 0x48, 0x18, 0x93, 0x25, 0xC8, 0x19, 0x46,
    ];
    let Ok((_h, answer)) = first_address(&m) else { panic!("parses") };
    let a = answer.expect("an AAAA answer");
    assert_eq!(
        a.ipv6,
        Some([0x26, 0x06, 0x28, 0x00, 0x02, 0x20, 0x00, 0x01, 0x02, 0x48, 0x18, 0x93, 0x25, 0xC8, 0x19, 0x46])
    );
    assert_eq!(a.ipv4, None);
}

#[test]
fn skips_a_cname_and_returns_the_following_a_record() {
    // The real-world common case: answer 1 is a CNAME, answer 2 is the A record.
    // The extractor must walk past the CNAME (using its rdlength) and return the A.
    let m = alloc::vec![
        0x12, 0x34, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x03, b'w', b'w', b'w', 0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
        0x03, b'c', b'o', b'm', 0x00, 0x00, 0x01, 0x00, 0x01,
        // answer 1: CNAME, rdata is a 2-byte compression pointer
        0xC0, 0x0C, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2C, 0x00, 0x02, 0xC0, 0x10,
        // answer 2: A -> 1.2.3.4
        0xC0, 0x0C, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2C, 0x00, 0x04, 1, 2, 3, 4,
    ];
    let Ok((_h, answer)) = first_address(&m) else { panic!("parses") };
    let a = answer.expect("the A answer after the CNAME");
    assert_eq!(a.ipv4, Some([1, 2, 3, 4]), "walked past the CNAME by its rdlength");
}
