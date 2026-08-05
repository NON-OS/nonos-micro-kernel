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

use super::types::{Container, PackErr, Section, SectionKind, ENTRY_LEN, HEADER_LEN, MAGIC};
use std::convert::TryInto;

pub fn decode(bytes: &[u8]) -> Result<(Container, usize), PackErr> {
    if bytes.len() < HEADER_LEN || &bytes[0..4] != MAGIC {
        return Err(PackErr::BadMagic);
    }
    if u16::from_be_bytes([bytes[4], bytes[5]]) != 1 {
        return Err(PackErr::BadVersion);
    }
    let count = u16::from_be_bytes([bytes[6], bytes[7]]) as usize;
    let table_end = HEADER_LEN + count * ENTRY_LEN;
    if bytes.len() < table_end {
        return Err(PackErr::Truncated);
    }
    let mut sections = Vec::with_capacity(count);
    let mut last_kind = 0u16;
    let mut trailer_off = table_end;
    for i in 0..count {
        let e = &bytes[HEADER_LEN + i * ENTRY_LEN..HEADER_LEN + (i + 1) * ENTRY_LEN];
        let kind_raw = u16::from_be_bytes(e[0..2].try_into().unwrap());
        let offset = u32::from_be_bytes(e[4..8].try_into().unwrap()) as usize;
        let length = u32::from_be_bytes(e[8..12].try_into().unwrap()) as usize;
        let kind = SectionKind::from_u16(kind_raw).ok_or(PackErr::UnknownKind)?;
        if kind_raw == last_kind {
            return Err(PackErr::Duplicate);
        }
        if kind_raw < last_kind {
            return Err(PackErr::OutOfOrder);
        }
        last_kind = kind_raw;
        let end = offset.checked_add(length).ok_or(PackErr::Truncated)?;
        if end > bytes.len() {
            return Err(PackErr::Truncated);
        }
        trailer_off = trailer_off.max(end);
        sections.push(Section { kind, bytes: bytes[offset..end].to_vec() });
    }
    Ok((Container { sections }, trailer_off))
}

pub fn section<'a>(c: &'a Container, k: SectionKind) -> Option<&'a [u8]> {
    c.sections.iter().find(|s| s.kind == k).map(|s| s.bytes.as_slice())
}
