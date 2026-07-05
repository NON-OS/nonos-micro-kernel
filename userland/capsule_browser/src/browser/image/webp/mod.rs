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

// A WebP decoder for the lossless VP8L path: bit reader, canonical Huffman,
// meta-Huffman groups, LZ77 with a color cache, and the four inverse
// transforms. Lossy VP8 payloads are not decoded here.

mod bitread;
mod code_len_code;
mod color_cache;
mod color_tf;
mod decode_image;
mod decode_pixels;
mod dist;
mod green_tf;
mod header;
mod hgroup;
mod huffman;
mod indexing;
mod lz77;
mod meta;
mod predict_fns;
mod predictor;
mod read_code_lengths;
mod read_huffman;
mod read_transform;
mod transform;
mod vp8l;

use crate::browser::image::store::Decoded;

// Decode a WebP container, dispatching the simple-lossless VP8L chunk. VP8X
// extended files are walked for their VP8L chunk; lossy VP8 is unsupported.
pub fn decode_webp(bytes: &[u8]) -> Option<Decoded> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return None;
    }
    let mut off = 12usize;
    while off + 8 <= bytes.len() {
        let fourcc = &bytes[off..off + 4];
        let size = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().ok()?) as usize;
        let end = off.checked_add(8)?.checked_add(size)?;
        let payload = bytes.get(off + 8..end.min(bytes.len()))?;
        if fourcc == b"VP8L" {
            return vp8l::decode_vp8l(payload);
        }
        // Even-padded chunk sizes; VP8X falls through to reach the VP8L chunk.
        off = end + (size & 1);
    }
    None
}
