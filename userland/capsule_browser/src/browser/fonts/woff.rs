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

use super::sfnt_header::sfnt_header;

// Unwrap a WOFF 1.0 container back into the sfnt the rasterizer parses:
// rebuild the header and table directory and inflate each table that was
// deflated. Anything that is not a WOFF passes through unchanged.
pub(super) fn unwrap_woff(bytes: Vec<u8>) -> Option<Vec<u8>> {
    if bytes.len() < 44 || &bytes[..4] != b"wOFF" {
        return Some(bytes);
    }
    let num = u16::from_be_bytes([bytes[12], bytes[13]]) as usize;
    if num == 0 || num > 64 || bytes.len() < 44 + num * 20 {
        return None;
    }
    let mut out = sfnt_header(&bytes[4..8], num);
    let mut tables: Vec<(usize, Vec<u8>, [u8; 4], u32)> = Vec::new();
    let mut offset = 12 + num * 16;
    for i in 0..num {
        let e = 44 + i * 20;
        let tag: [u8; 4] = bytes[e..e + 4].try_into().ok()?;
        let src_off = u32::from_be_bytes(bytes[e + 4..e + 8].try_into().ok()?) as usize;
        let comp_len = u32::from_be_bytes(bytes[e + 8..e + 12].try_into().ok()?) as usize;
        let orig_len = u32::from_be_bytes(bytes[e + 12..e + 16].try_into().ok()?) as usize;
        let checksum = u32::from_be_bytes(bytes[e + 16..e + 20].try_into().ok()?);
        let src = bytes.get(src_off..src_off.checked_add(comp_len)?)?;
        let data = if comp_len < orig_len {
            let inflated = nonos_inflate::zlib(src)?;
            if inflated.len() != orig_len {
                return None;
            }
            inflated
        } else {
            src.to_vec()
        };
        let padded = (data.len() + 3) & !3;
        tables.push((offset, data, tag, checksum));
        offset += padded;
    }
    for (off, data, tag, checksum) in &tables {
        out.extend_from_slice(tag);
        out.extend_from_slice(&checksum.to_be_bytes());
        out.extend_from_slice(&(*off as u32).to_be_bytes());
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    }
    for (_, data, _, _) in &tables {
        out.extend_from_slice(data);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    }
    Some(out)
}
