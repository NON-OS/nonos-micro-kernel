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

//! Split a firmware image into the memory sections the download programs into
//! the card. After the 64-byte header the body is DMEM, then IMEM, then an
//! optional EMEM, each followed by an 8-byte checksum the card verifies. This
//! turns the header sizes into concrete (on-chip address, image slice) sections
//! the DDMA loop walks; it is pure arithmetic over the header, checked against
//! the real firmware in `rtl8821ce_proofs`.

use super::header::{FwHeader, FW_HDR_LEN};

/// The 8-byte checksum appended to each downloaded section.
pub const SECTION_CHKSUM_LEN: usize = 8;
/// Bit 4 of `mem_usage` marks an EMEM section as present.
const MEM_USAGE_EMEM: u8 = 1 << 4;
/// The largest chunk the download copies in one DDMA transfer.
pub const MAX_CHUNK: usize = 0x1000;
/// The section addresses in the image carry bit 31 as an image-format marker,
/// not part of the on-chip address. rtw88 clears it before using the value as a
/// DDMA destination (`addr &= ~BIT(31)` in `start_download_firmware`), which
/// turns 0x8020_0000 into the real on-chip DMEM base 0x0020_0000.
const OCP_ADDR_MASK: u32 = 0x7FFF_FFFF;

/// One memory section to download: where it lands on the chip and the bytes of
/// the image it covers (size includes the trailing checksum).
#[derive(Clone, Copy)]
pub struct Section {
    pub dst: u32,
    pub offset: usize,
    pub len: usize,
}

/// The DMEM, IMEM and (when present) EMEM sections of `fw`, in download order.
/// Returns `None` if a section would run past the image, so a truncated or
/// mismatched image is rejected before any register is touched. `count` is 2
/// or 3.
pub fn sections(hdr: &FwHeader, fw_len: usize) -> Option<([Section; 3], usize)> {
    let mut out = [Section { dst: 0, offset: 0, len: 0 }; 3];
    let mut n = 0;
    let mut offset = FW_HDR_LEN;

    let mut push = |dst: u32, size: usize| -> Option<()> {
        let len = size + SECTION_CHKSUM_LEN;
        let end = offset.checked_add(len)?;
        if end > fw_len {
            return None;
        }
        out[n] = Section { dst: dst & OCP_ADDR_MASK, offset, len };
        offset = end;
        n += 1;
        Some(())
    };

    push(hdr.dmem_addr, hdr.dmem_size as usize)?;
    push(hdr.imem_addr, hdr.imem_size as usize)?;
    if hdr.mem_usage & MEM_USAGE_EMEM != 0 {
        push(hdr.emem_addr, hdr.emem_size as usize)?;
    }
    Some((out, n))
}
