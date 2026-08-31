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

use crate::doc::export::pdf::fmt::{push_f32, push_usize};
use crate::doc::export::pdf::objects::FONTS;
use crate::doc::page::PageMetrics;

pub fn page_obj(contents: usize, pm: &PageMetrics) -> Vec<u8> {
    let mut out = Vec::from(&b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 "[..]);
    push_f32(&mut out, pm.width);
    out.push(b' ');
    push_f32(&mut out, pm.height);
    out.extend_from_slice(b"] /Resources << /Font <<");
    for i in 0..FONTS.len() {
        out.extend_from_slice(b" /F");
        push_usize(&mut out, i + 1);
        out.push(b' ');
        push_usize(&mut out, 3 + i);
        out.extend_from_slice(b" 0 R");
    }
    out.extend_from_slice(b" >> >> /Contents ");
    push_usize(&mut out, contents);
    out.extend_from_slice(b" 0 R >>");
    out
}

pub fn stream_obj(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::from(&b"<< /Length "[..]);
    push_usize(&mut out, data.len());
    out.extend_from_slice(b" >>\nstream\n");
    out.extend_from_slice(data);
    out.extend_from_slice(b"\nendstream");
    out
}
