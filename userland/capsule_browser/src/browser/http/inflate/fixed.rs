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

use alloc::vec::Vec;

use super::bits::Bits;
use super::codes::codes;
use super::huff::build;

pub fn fixed(b: &mut Bits, out: &mut Vec<u8>) -> Option<()> {
    let mut ll = [0u8; 288];
    for item in ll.iter_mut().take(144) {
        *item = 8;
    }
    for item in ll.iter_mut().take(256).skip(144) {
        *item = 9;
    }
    for item in ll.iter_mut().take(280).skip(256) {
        *item = 7;
    }
    for item in ll.iter_mut().skip(280) {
        *item = 8;
    }
    let lit = build(&ll);
    let dist = build(&[5u8; 30]);
    codes(b, out, &lit, &dist)
}
