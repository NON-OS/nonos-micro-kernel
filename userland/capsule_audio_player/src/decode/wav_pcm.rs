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

pub fn parse_riff(bytes: &[u8]) -> Result<(u32, u8, u16, usize, usize), &'static str> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("bad riff header");
    }
    let mut off = 12usize;
    let mut rate = 0u32;
    let mut channels = 0u8;
    let mut bits = 0u16;
    let mut data_off = 0usize;
    let mut data_len = 0usize;
    while off + 8 <= bytes.len() {
        let id = &bytes[off..off + 4];
        let sz = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap()) as usize;
        let body = off + 8;
        if id == b"fmt " && body + 16 <= bytes.len() {
            channels = bytes[body + 2];
            rate = u32::from_le_bytes(bytes[body + 4..body + 8].try_into().unwrap());
            bits = u16::from_le_bytes(bytes[body + 14..body + 16].try_into().unwrap());
        } else if id == b"data" {
            data_off = body;
            data_len = sz.min(bytes.len().saturating_sub(body));
        }
        off = body + sz + (sz & 1);
    }
    if rate == 0 || channels == 0 || bits == 0 || data_len == 0 {
        return Err("missing fmt or data chunk");
    }
    Ok((rate, channels, bits, data_off, data_len))
}

pub fn decode_sample(b: &[u8], bits: u16) -> i16 {
    match bits {
        8 => ((b[0] as i16) - 128) << 8,
        16 => i16::from_le_bytes([b[0], b[1]]),
        24 => i16::from_le_bytes([b[1], b[2]]),
        _ => 0,
    }
}
