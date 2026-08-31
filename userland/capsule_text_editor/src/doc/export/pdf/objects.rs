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

use crate::doc::export::pdf::fmt::push_usize;

pub const FONTS: [&str; 4] = ["Helvetica", "Helvetica-Bold", "Courier", "Courier-Bold"];
pub const FIRST_PAGE_OBJ: usize = 7;

pub fn page_obj_id(index: usize) -> usize {
    FIRST_PAGE_OBJ + 2 * index
}

pub fn catalog() -> Vec<u8> {
    Vec::from(&b"<< /Type /Catalog /Pages 2 0 R >>"[..])
}

pub fn pages(count: usize) -> Vec<u8> {
    let mut out = Vec::from(&b"<< /Type /Pages /Count "[..]);
    push_usize(&mut out, count);
    out.extend_from_slice(b" /Kids [");
    for i in 0..count {
        out.push(b' ');
        push_usize(&mut out, page_obj_id(i));
        out.extend_from_slice(b" 0 R");
    }
    out.extend_from_slice(b" ] >>");
    out
}

pub fn font(name: &str) -> Vec<u8> {
    let mut out = Vec::from(&b"<< /Type /Font /Subtype /Type1 /BaseFont /"[..]);
    out.extend_from_slice(name.as_bytes());
    out.extend_from_slice(b" /Encoding /WinAnsiEncoding >>");
    out
}
