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
//! pkt-line framing and the request a fetch sends.

use nonos_git::{encode_pkt, read_pkt, want_request, ObjectId, Pkt};

#[test]
fn pkt_lines_round_trip() {
    let mut out = Vec::new();
    encode_pkt(b"want abc\n", &mut out);
    // Nine payload bytes plus the four-byte header.
    assert_eq!(&out[..4], b"000d");
    let (pkt, used) = read_pkt(&out).expect("read back");
    assert_eq!(used, 13);
    match pkt {
        Pkt::Data(d) => assert_eq!(d, b"want abc\n"),
        Pkt::Flush => panic!("expected data"),
    }
}

#[test]
fn a_flush_packet_reads_as_flush() {
    let (pkt, used) = read_pkt(b"0000").expect("flush");
    assert_eq!(used, 4);
    assert!(matches!(pkt, Pkt::Flush));
}

#[test]
fn a_want_request_is_framed_the_way_git_sends_it() {
    // GitHub answers this exact body with 200 and a pack.
    let id = ObjectId::from_hex("7fd1a60b01f91b314f59955a4e4d4e80d8edf11d").unwrap();
    let body = want_request(&[id], 1);
    let text = String::from_utf8_lossy(&body).into_owned();
    assert!(text.contains("want 7fd1a60b01f91b314f59955a4e4d4e80d8edf11d no-progress ofs-delta\n"));
    assert!(text.contains("deepen 1\n"));
    assert!(text.ends_with("0009done\n"));
}
