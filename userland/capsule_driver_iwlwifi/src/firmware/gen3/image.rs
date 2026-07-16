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

//! Parse an AX210-class firmware image for the gen3 self-load. The framing is
//! the common iwlwifi TLV format: an 88-byte ucode header, then
//! `[type:u32][len:u32][payload]` records padded to four bytes. Gen3 firmware
//! carries its runtime code as sections under type 19 and the boot ROM's image
//! loader under type 52. Each runtime section payload is a u32 device load
//! offset followed by that section's bytes. The offsets, counts and image
//! loader length are pinned to the real so-a0-gf-a0 image by the gen3 proofs.
//!
//! The constants here are gen3-local on purpose: the module stays self-contained
//! so it compiles the same way whether pulled into the capsule or the proofs.

const TLV_HEADER_LEN: usize = 88;
const TLV_SEC_RT: u32 = 19;
const TLV_IML: u32 = 52;

/// One runtime firmware section: the device load offset and the section bytes
/// (the u32 offset prefix already stripped).
#[derive(Clone, Copy)]
pub struct Section<'a> {
    pub load_offset: u32,
    pub data: &'a [u8],
}

/// The image loader blob the ROM runs to fetch the firmware, or None if the
/// image carries no type-52 record.
pub fn find_iml(blob: &[u8]) -> Option<&[u8]> {
    let mut walk = TlvWalk::new(blob)?;
    while let Some((ty, payload)) = walk.next_record() {
        if ty == TLV_IML {
            return Some(payload);
        }
    }
    None
}

/// Iterate the runtime (type 19) sections in the order they appear.
pub fn sections(blob: &[u8]) -> SectionIter<'_> {
    SectionIter { walk: TlvWalk::new(blob) }
}

/// Walks the runtime sections, yielding each as a load offset and its bytes.
pub struct SectionIter<'a> {
    walk: Option<TlvWalk<'a>>,
}

impl<'a> Iterator for SectionIter<'a> {
    type Item = Section<'a>;
    fn next(&mut self) -> Option<Section<'a>> {
        let walk = self.walk.as_mut()?;
        while let Some((ty, payload)) = walk.next_record() {
            // A runtime section is a u32 load offset then the section body.
            if ty == TLV_SEC_RT && payload.len() >= 4 {
                let load_offset = u32::from_le_bytes(payload[0..4].try_into().ok()?);
                return Some(Section { load_offset, data: &payload[4..] });
            }
        }
        None
    }
}

// A bounds-checked walk over the TLV records after the ucode header.
struct TlvWalk<'a> {
    blob: &'a [u8],
    off: usize,
}

impl<'a> TlvWalk<'a> {
    fn new(blob: &'a [u8]) -> Option<TlvWalk<'a>> {
        if blob.len() < TLV_HEADER_LEN {
            return None;
        }
        Some(TlvWalk { blob, off: TLV_HEADER_LEN })
    }

    fn next_record(&mut self) -> Option<(u32, &'a [u8])> {
        if self.off + 8 > self.blob.len() {
            return None;
        }
        let ty = le32(self.blob, self.off)?;
        let len = le32(self.blob, self.off + 4)? as usize;
        let body = self.off + 8;
        let end = body.checked_add(len)?;
        if end > self.blob.len() {
            return None;
        }
        self.off = body + ((len + 3) & !3);
        Some((ty, &self.blob[body..end]))
    }
}

fn le32(d: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_le_bytes(d.get(o..o + 4)?.try_into().ok()?))
}
