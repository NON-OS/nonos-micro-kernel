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

//! Known answer tests from RFC 8452 Appendix A.
//!
//! POLYVAL is worth pinning because a wrong implementation is not obviously
//! wrong: it produces a stable, self consistent tag that verifies against
//! itself and against nothing else. Only a published vector catches it.

use super::Polyval;

fn hex16(s: &str) -> [u8; 16] {
    let b = s.as_bytes();
    let mut out = [0u8; 16];
    let mut i = 0;
    while i < 16 {
        out[i] = nib(b[i * 2]) << 4 | nib(b[i * 2 + 1]);
        i += 1;
    }
    out
}

fn nib(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => 0,
    }
}

/// RFC 8452 Appendix A, the worked POLYVAL example: one key, two blocks.
#[test]
fn rfc8452_two_blocks() {
    let h = hex16("25629347589242761d31f826ba4b757b");
    let x1 = hex16("4f4f95668c83dfb6401762bb2d01a262");
    let x2 = hex16("d1a24ddd2721d006bbe45f20d3c9f362");

    let mut p = Polyval::new(&h);
    p.update(&x1);
    p.update(&x2);
    assert_eq!(p.finalize(), hex16("f7a3b47b846119fae5b7866cf5e5b77e"));
}

/// A single block against the same key. Absorbing one block must not depend
/// on a second arriving.
#[test]
fn rfc8452_single_block() {
    let h = hex16("25629347589242761d31f826ba4b757b");
    let mut p = Polyval::new(&h);
    p.update(&hex16("4f4f95668c83dfb6401762bb2d01a262"));
    assert_eq!(p.finalize(), hex16("cedac64537ff50989c16011551086d77"));
}

/// The empty message. No blocks absorbed leaves the accumulator at zero.
#[test]
fn empty_is_zero() {
    let p = Polyval::new(&hex16("25629347589242761d31f826ba4b757b"));
    assert_eq!(p.finalize(), [0u8; 16]);
}
