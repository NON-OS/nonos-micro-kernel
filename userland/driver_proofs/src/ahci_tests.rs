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

use crate::constants::ata::MAX_SECTORS;
use crate::server::handlers::parse_rw;

extern crate alloc;
use alloc::vec::Vec;

// A block read/write request body is attacker-controlled (LBA + sector count).
// Parsing must never panic, and an accepted request must never reach beyond the
// disk: the sector count is bounded and lba + count never overflows or exceeds
// the device capacity.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn rw_parse_never_panics_and_requests_stay_within_the_disk() {
    for seed in 1..150_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 20) as usize;
        let body: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        let capacity = xorshift(&mut s);

        if let Ok((lba, nsectors)) = parse_rw(&body, capacity) {
            assert!((1..=MAX_SECTORS).contains(&nsectors), "sector count out of range");
            let last = lba.checked_add(nsectors as u64);
            assert!(last.is_some(), "lba + count overflowed");
            assert!(last.unwrap() <= capacity, "request reaches past the disk");
        }
    }
}

#[test]
fn short_bodies_are_rejected_not_panicked() {
    for len in 0..12usize {
        let body = vec![0xffu8; len];
        assert!(parse_rw(&body, u64::MAX).is_err(), "a short request must be rejected");
    }
}
