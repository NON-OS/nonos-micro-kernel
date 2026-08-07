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

use crate::types::{PkgErr, Sections, ENTRY_LEN, HEADER_LEN, MAGIC};

pub fn parse(bytes: &[u8]) -> Result<(Sections<'_>, usize), PkgErr> {
    if bytes.len() < HEADER_LEN {
        return Err(PkgErr::Short);
    }
    if &bytes[0..4] != MAGIC {
        return Err(PkgErr::BadMagic);
    }
    if be16(bytes, 4) != 1 {
        return Err(PkgErr::BadVersion);
    }
    if be16(bytes, 6) != 4 {
        return Err(PkgErr::BadCount);
    }
    let table_end = HEADER_LEN + 4 * ENTRY_LEN;
    if bytes.len() < table_end {
        return Err(PkgErr::Short);
    }
    let mut slots: [Option<&[u8]>; 4] = [None; 4];
    let mut last = 0u16;
    let mut trailer_off = table_end;
    for i in 0..4 {
        let e = HEADER_LEN + i * ENTRY_LEN;
        let kind = be16(bytes, e);
        if kind == last {
            return Err(PkgErr::DupKind);
        }
        if kind < last {
            return Err(PkgErr::OutOfOrder);
        }
        last = kind;
        let offset = be32(bytes, e + 4) as usize;
        let length = be32(bytes, e + 8) as usize;
        let end = offset.checked_add(length).ok_or(PkgErr::BadExtent)?;
        if end > bytes.len() {
            return Err(PkgErr::BadExtent);
        }
        if trailer_off < end {
            trailer_off = end;
        }
        if (1..=4).contains(&kind) {
            slots[(kind - 1) as usize] = Some(&bytes[offset..end]);
        }
    }
    let (Some(manifest), Some(elf), Some(id_cert), Some(zk_trailer)) =
        (slots[0], slots[1], slots[2], slots[3])
    else {
        return Err(PkgErr::MissingSection);
    };
    Ok((Sections { manifest, elf, id_cert, zk_trailer }, trailer_off))
}

fn be16(b: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([b[i], b[i + 1]])
}

fn be32(b: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}
