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

use crate::term::util::format_u64;

pub struct Tally {
    pub ok: u32,
    pub skipped: u32,
    pub failed: u32,
}

pub fn file_line(name: &[u8], size: usize) -> Vec<u8> {
    let mut l = Vec::new();
    l.extend_from_slice(name);
    l.extend_from_slice(b"  ");
    push_num(&mut l, size as u64);
    l.extend_from_slice(b" bytes");
    l
}

pub fn summary(t: &Tally) -> Vec<u8> {
    let mut l = Vec::new();
    push_num(&mut l, t.ok as u64);
    l.extend_from_slice(b" ok, ");
    push_num(&mut l, t.skipped as u64);
    l.extend_from_slice(b" skipped, ");
    push_num(&mut l, t.failed as u64);
    l.extend_from_slice(b" failed");
    l
}

fn push_num(l: &mut Vec<u8>, n: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(n, &mut buf);
    l.extend_from_slice(&buf[..k]);
}
