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

//! The answer this capsule writes back.

use alloc::vec::Vec;

/// Response, recursion desired carried back, recursion available.
const FLAGS: u16 = 0x8180;

/// A short life.
const TTL: u32 = 60;

const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

/// `question` is the query's bytes from the header through the qclass.
pub fn build(question: &[u8], addr: Option<[u8; 4]>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(question.len() + 16);
    out.extend_from_slice(&question[..2]);
    out.extend_from_slice(&FLAGS.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&u16::from(addr.is_some()).to_be_bytes());
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&question[12..]);
    let Some(addr) = addr else {
        return out;
    };
    // The name is not written again.
    out.extend_from_slice(&0xC00Cu16.to_be_bytes());
    out.extend_from_slice(&TYPE_A.to_be_bytes());
    out.extend_from_slice(&CLASS_IN.to_be_bytes());
    out.extend_from_slice(&TTL.to_be_bytes());
    out.extend_from_slice(&4u16.to_be_bytes());
    out.extend_from_slice(&addr);
    out
}
