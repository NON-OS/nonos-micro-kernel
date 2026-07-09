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

use crate::elf::types::ProgramHeader;
use crate::elf::{parse_header, program_header_bounds};
use core::mem::size_of;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

// A capsule ELF is untrusted: its header names the offset, count and entry size
// of the program-header table. Parsing must never panic, and an accepted table
// must fit inside the file with no integer overflow in `phoff + phnum*phentsize`.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

// A 64-byte ELF64 header (padded to `total`) with the given phoff/phnum/phentsize.
fn craft(total: usize, phoff: u64, phnum: u16, phentsize: u16) -> Vec<u8> {
    let mut b = vec![0u8; total.max(64)];
    b[32..40].copy_from_slice(&phoff.to_le_bytes());
    b[54..56].copy_from_slice(&phentsize.to_le_bytes());
    b[56..58].copy_from_slice(&phnum.to_le_bytes());
    b
}

#[test]
fn short_headers_are_rejected_not_panicked() {
    for len in 0..64usize {
        assert!(parse_header(&vec![0u8; len]).is_err(), "a truncated header must be rejected");
    }
}

#[test]
fn program_header_table_never_overflows_or_escapes_the_file() {
    let valid_phsize = size_of::<ProgramHeader>() as u16;
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..300_000 {
        let total = 64 + (xorshift(&mut s) % 8192) as usize;
        let phoff = match xorshift(&mut s) % 4 {
            0 => xorshift(&mut s),               // fully arbitrary (often huge)
            1 => (xorshift(&mut s) % 8192) + 64, // plausibly in-file
            2 => u64::MAX - (xorshift(&mut s) % 128), // near overflow
            _ => 0,
        };
        let phnum = (xorshift(&mut s) & 0xffff) as u16;
        // Half the time use the size that reaches the bounds computation.
        let phentsize =
            if xorshift(&mut s) & 1 == 0 { valid_phsize } else { (xorshift(&mut s) & 0xffff) as u16 };

        let bytes = craft(total, phoff, phnum, phentsize);
        if let Ok(header) = parse_header(&bytes) {
            if let Ok((off, size, count)) = program_header_bounds(&bytes, &header) {
                if count > 0 {
                    let table = size.checked_mul(count);
                    assert!(table.is_some(), "phentsize * phnum overflowed");
                    let end = off.checked_add(table.unwrap());
                    assert!(end.is_some(), "phoff + table_bytes overflowed");
                    assert!(end.unwrap() <= bytes.len(), "program header table escapes the file");
                }
            }
        }
    }
}
