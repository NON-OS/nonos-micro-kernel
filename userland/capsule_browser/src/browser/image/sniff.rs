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

// Format detection and dimension probe from an image header, so the raster
// buffer can be sized before the full decode runs.

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Format {
    Png,
    Jpeg,
    Bmp,
    Gif,
    Webp,
}

pub(super) struct Probe {
    pub format: Format,
    pub w: u32,
    pub h: u32,
}

fn be16(b: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_be_bytes(b.get(i..i + 2)?.try_into().ok()?))
}

fn be32(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(i..i + 4)?.try_into().ok()?))
}

fn le32(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?))
}

fn le16(b: &[u8], i: usize) -> Option<u32> {
    Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?) as u32)
}

pub(super) fn probe(b: &[u8]) -> Option<Probe> {
    const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    if b.starts_with(&PNG) {
        return Some(Probe { format: Format::Png, w: be32(b, 16)?, h: be32(b, 20)? });
    }
    if b.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return jpeg_dims(b);
    }
    if b.starts_with(b"BM") {
        return Some(Probe { format: Format::Bmp, w: le32(b, 18)?, h: le32(b, 22)? });
    }
    if b.starts_with(b"GIF87a") || b.starts_with(b"GIF89a") {
        return Some(Probe { format: Format::Gif, w: le16(b, 6)?, h: le16(b, 8)? });
    }
    if b.len() >= 16 && &b[0..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        return webp_dims(b);
    }
    None
}

// Walk JPEG segments to the first frame header and read its dimensions.
fn jpeg_dims(b: &[u8]) -> Option<Probe> {
    let mut i = 2usize;
    while i + 4 <= b.len() {
        if b[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = b[i + 1];
        // Standalone markers carry no length payload.
        if marker == 0xD8 || marker == 0xD9 || (0xD0..=0xD7).contains(&marker) || marker == 0x01 {
            i += 2;
            continue;
        }
        let len = be16(b, i + 2)? as usize;
        // SOFn markers hold the frame's height then width.
        if matches!(marker, 0xC0..=0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF) {
            let h = be16(b, i + 5)? as u32;
            let w = be16(b, i + 7)? as u32;
            return Some(Probe { format: Format::Jpeg, w, h });
        }
        i += 2 + len;
    }
    None
}

// WebP dimensions from the first chunk after the RIFF/WEBP header. VP8X
// carries a 24-bit canvas size, VP8L packs 14-bit dims after a signature
// byte, and lossy VP8 reads them from the keyframe header.
fn webp_dims(b: &[u8]) -> Option<Probe> {
    let fourcc = b.get(12..16)?;
    match fourcc {
        b"VP8X" => {
            let w = 1 + (le16(b, 24)? | ((b.get(26).copied()? as u32) << 16)) & 0xFF_FFFF;
            let h = 1 + (le16(b, 27)? | ((b.get(29).copied()? as u32) << 16)) & 0xFF_FFFF;
            Some(Probe { format: Format::Webp, w, h })
        }
        b"VP8L" => {
            let bits = le32(b, 21)?;
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            Some(Probe { format: Format::Webp, w, h })
        }
        b"VP8 " => {
            // Skip the 10-byte frame tag and the 3-byte start code to the
            // 14-bit width and height words.
            let w = le16(b, 26)? & 0x3FFF;
            let h = le16(b, 28)? & 0x3FFF;
            Some(Probe { format: Format::Webp, w, h })
        }
        _ => None,
    }
}
