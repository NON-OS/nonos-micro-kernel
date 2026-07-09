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

//! Robustness proofs for the raw wire parsers (Ethernet, IPv4) over millions of
//! attacker-shaped packets: never panic, and every accepted packet yields an
//! in-bounds payload. The `kani_proofs` module proves the same over all inputs.

use alloc::vec;
use alloc::vec::Vec;

use crate::net::eth::frame::{payload_of, EthHeader, HDR_LEN};
use crate::net::ipv4::checksum::fold;
use crate::net::ipv4::parse::parse as ipv4_parse;
use crate::net::udp::build::{build as udp_build, BuildRequest};
use crate::net::udp::parse::parse as udp_parse;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn eth_parse_never_panics_and_roundtrips() {
    let mut rng = 0x00c0_ffee_1234_5678u64;
    for _ in 0..2_000_000 {
        let len = (next(&mut rng) % 40) as usize;
        let buf: Vec<u8> = (0..len).map(|_| next(&mut rng) as u8).collect();
        match EthHeader::parse(&buf) {
            Some(h) => {
                // A valid header round-trips: writing it back reproduces the
                // first 14 bytes, and the payload is exactly the remainder.
                let mut out = [0u8; HDR_LEN];
                assert!(h.write(&mut out));
                assert_eq!(&out[..], &buf[..HDR_LEN]);
                assert_eq!(payload_of(&buf).unwrap().len(), buf.len() - HDR_LEN);
            }
            None => assert!(buf.len() < HDR_LEN),
        }
    }
}

#[test]
fn ipv4_parse_never_panics_and_payload_in_bounds() {
    let mut rng = 0x00a1_1ce5_badc_0de1u64;
    // Pure random packets.
    for _ in 0..2_000_000 {
        let len = (next(&mut rng) % 48) as usize;
        let buf: Vec<u8> = (0..len).map(|_| next(&mut rng) as u8).collect();
        if let Ok((_h, payload)) = ipv4_parse(&buf) {
            assert!(payload.len() <= buf.len(), "payload escaped the frame");
        }
    }
    // Well-formed headers with a correct checksum, to drive the deep validation
    // branches (total-length, fragment, checksum) rather than bailing early.
    for _ in 0..500_000 {
        let mut buf = vec![0u8; 20];
        buf[0] = 0x45; // IPv4, IHL = 5 words
        buf[2..4].copy_from_slice(&20u16.to_be_bytes()); // total length
        let frag = (next(&mut rng) as u16) & 0x3FFF;
        buf[6..8].copy_from_slice(&frag.to_be_bytes());
        buf[8] = next(&mut rng) as u8; // ttl
        buf[9] = next(&mut rng) as u8; // protocol
        for b in buf.iter_mut().skip(12).take(8) {
            *b = next(&mut rng) as u8; // src + dst
        }
        buf[10] = 0;
        buf[11] = 0;
        let ck = fold(&buf[..20]);
        buf[10..12].copy_from_slice(&ck.to_be_bytes());
        if let Ok((_h, payload)) = ipv4_parse(&buf) {
            assert!(payload.len() <= buf.len());
        }
    }
}

#[test]
fn udp_parse_never_panics_and_payload_in_bounds() {
    let mut rng = 0x00dd_ee11_2233_4455u64;
    let src = [10u8, 0, 2, 15];
    let dst = [10u8, 0, 2, 2];
    // Pure random segments.
    for _ in 0..2_000_000 {
        let len = (next(&mut rng) % 48) as usize;
        let seg: Vec<u8> = (0..len).map(|_| next(&mut rng) as u8).collect();
        if let Ok((_h, payload)) = udp_parse(&src, &dst, &seg) {
            assert!(payload.len() <= seg.len(), "payload escaped the segment");
        }
    }
    // Well-formed segments with the checksum field zeroed (checksum unused), so
    // parse reaches the payload branch instead of bailing at validation.
    for _ in 0..500_000 {
        let plen = (next(&mut rng) % 40) as usize;
        let total = 8 + plen;
        let mut seg = vec![0u8; total];
        seg[0..2].copy_from_slice(&(next(&mut rng) as u16).to_be_bytes()); // src port
        seg[2..4].copy_from_slice(&(next(&mut rng) as u16).to_be_bytes()); // dst port
        seg[4..6].copy_from_slice(&(total as u16).to_be_bytes()); // length
                                                                  // checksum bytes [6..8] stay zero => validation skipped
        for b in seg.iter_mut().skip(8) {
            *b = next(&mut rng) as u8;
        }
        if let Ok((_h, payload)) = udp_parse(&src, &dst, &seg) {
            assert_eq!(payload.len(), plen);
            assert!(payload.len() <= seg.len());
        }
    }
}

#[test]
fn udp_build_then_parse_recovers_ports_and_payload() {
    let mut rng = 0x00bb_11dd_ee22_ff33u64;
    let src = [10u8, 0, 2, 15];
    let dst = [10u8, 0, 2, 2];
    for _ in 0..500_000 {
        let plen = (next(&mut rng) % 64) as usize;
        let payload: Vec<u8> = (0..plen).map(|_| next(&mut rng) as u8).collect();
        let req = BuildRequest {
            src,
            dst,
            src_port: next(&mut rng) as u16,
            dst_port: next(&mut rng) as u16,
            payload: &payload,
        };
        let mut out = vec![0u8; 8 + plen];
        let n = match udp_build(&req, &mut out) {
            Ok(n) => n,
            Err(_) => panic!("build of a well-sized segment must succeed"),
        };
        assert_eq!(n, 8 + plen);
        // A built segment carries a valid checksum, so parse must accept it and
        // recover the same ports and payload it was built from.
        let (h, got) = match udp_parse(&src, &dst, &out[..n]) {
            Ok(v) => v,
            Err(_) => panic!("a freshly built segment must parse"),
        };
        assert_eq!(h.src_port, req.src_port);
        assert_eq!(h.dst_port, req.dst_port);
        assert_eq!(got, &payload[..]);
    }
}
