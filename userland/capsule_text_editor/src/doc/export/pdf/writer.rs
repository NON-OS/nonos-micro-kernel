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

use crate::doc::export::pdf::fmt::{push_offset, push_usize};

pub fn assemble(bodies: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n");
    out.extend_from_slice(b"%\xE2\xE3\xCF\xD3\n");
    let mut offsets = Vec::with_capacity(bodies.len());
    for (i, body) in bodies.iter().enumerate() {
        offsets.push(out.len());
        push_usize(&mut out, i + 1);
        out.extend_from_slice(b" 0 obj\n");
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }
    let start = out.len();
    push_xref(&mut out, &offsets);
    push_trailer(&mut out, bodies.len() + 1, start);
    out
}

fn push_xref(out: &mut Vec<u8>, offsets: &[usize]) {
    out.extend_from_slice(b"xref\n0 ");
    push_usize(out, offsets.len() + 1);
    out.extend_from_slice(b"\n0000000000 65535 f \n");
    for off in offsets {
        push_offset(out, *off);
        out.extend_from_slice(b" 00000 n \n");
    }
}

fn push_trailer(out: &mut Vec<u8>, size: usize, start: usize) {
    out.extend_from_slice(b"trailer\n<< /Size ");
    push_usize(out, size);
    out.extend_from_slice(b" /Root 1 0 R >>\nstartxref\n");
    push_usize(out, start);
    out.extend_from_slice(b"\n%%EOF");
}
