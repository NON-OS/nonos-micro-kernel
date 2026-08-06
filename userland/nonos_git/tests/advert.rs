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
//! The ref advertisement, against what GitHub actually serves.
//!
//! `advert_live.bin` is the service banner, the HEAD packet and the
//! refs/heads/master packet from a real `GET /info/refs` response for
//! octocat/Hello-World, reassembled without the other 3370 refs. Every byte
//! of every packet is what GitHub sent.

use nonos_git::{parse_advertisement, WireError};

const ADVERT: &[u8] = include_bytes!("data/advert_live.bin");

#[test]
fn reads_a_real_advertisement() {
    let refs = parse_advertisement(ADVERT).expect("real advertisement must parse");
    assert!(!refs.is_empty());
    // The first entry GitHub advertises is HEAD, at the default branch tip.
    assert_eq!(refs[0].name, "HEAD");
    assert_eq!(refs[0].id.to_hex(), "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d");
    assert!(refs.iter().all(|r| r.id.to_hex().len() == 40));
}

#[test]
fn a_non_smart_response_is_refused() {
    // A dumb-protocol server answers without the service banner.
    assert_eq!(parse_advertisement(b"0000").err(), Some(WireError::NotSmartHttp));
    assert_eq!(parse_advertisement(b"").err(), Some(WireError::Truncated));
    assert_eq!(parse_advertisement(b"zzzz").err(), Some(WireError::Length));
}
