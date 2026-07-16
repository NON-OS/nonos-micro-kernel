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

//! Parse the 64-byte header of an rtw88 firmware image. The image is
//! `[header][dmem][imem][emem]`; the header carries the signature, version, the
//! H2C command-format version the card expects, and the size and target
//! address of each memory section the download splits the body into. Field
//! offsets follow the rtw88 `rtw_fw_hdr` layout; the parser is checked against
//! the real RTL8821CE firmware bytes in `rtl8821ce_proofs`.

/// The firmware header length, and the offset of the firmware body.
pub const FW_HDR_LEN: usize = 64;

/// The signature an RTL8821C firmware image carries in its first two bytes.
pub const CHIP_SIGNATURE_8821C: u16 = 0x8821;

/// The parsed header. Sizes and addresses drive the section-by-section
/// download; the signature identifies the chip the image is for.
#[derive(Clone, Copy)]
pub struct FwHeader {
    pub signature: u16,
    pub version: u16,
    pub subversion: u8,
    /// Memory-usage flags. Bit 4 set means the image carries an EMEM section.
    pub mem_usage: u8,
    pub h2c_fmt_ver: u16,
    pub dmem_addr: u32,
    pub dmem_size: u32,
    pub imem_addr: u32,
    pub imem_size: u32,
    pub emem_addr: u32,
    pub emem_size: u32,
}

impl FwHeader {
    /// True when this image is an RTL8821C firmware the driver can load: the
    /// chip signature matches and the header names a non-zero H2C command format
    /// (the version the 8051 will speak once running). Guards against a corrupt
    /// or wrong-chip embed before any register is touched.
    pub fn is_for_8821c(&self) -> bool {
        self.signature == CHIP_SIGNATURE_8821C && self.h2c_fmt_ver != 0
    }

    /// The firmware version and subversion, for the bring-up console.
    pub fn version(&self) -> (u16, u8) {
        (self.version, self.subversion)
    }
}

/// Parse the header, or `None` if the image is shorter than one header. The
/// caller checks the signature against the chip it is driving.
pub fn parse(fw: &[u8]) -> Option<FwHeader> {
    if fw.len() < FW_HDR_LEN {
        return None;
    }
    Some(FwHeader {
        signature: le16(fw, 0x00),
        version: le16(fw, 0x04),
        subversion: fw[0x06],
        mem_usage: fw[0x18],
        h2c_fmt_ver: le16(fw, 0x1C),
        dmem_addr: le32(fw, 0x20),
        dmem_size: le32(fw, 0x24),
        imem_size: le32(fw, 0x30),
        emem_size: le32(fw, 0x34),
        emem_addr: le32(fw, 0x38),
        imem_addr: le32(fw, 0x3C),
    })
}

fn le16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn le32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}
