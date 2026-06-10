// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Lz4,
    Zlib,
    Lzma,
}

pub fn compress_firmware(data: &[u8], ct: CompressionType) -> alloc::vec::Vec<u8> {
    match ct {
        CompressionType::None | CompressionType::Lzma => data.to_vec(),
        CompressionType::Lz4 => lz4_compress(data),
        CompressionType::Zlib => zlib_compress(data),
    }
}
pub fn decompress_firmware(
    data: &[u8],
    ct: CompressionType,
) -> Result<alloc::vec::Vec<u8>, &'static str> {
    match ct {
        CompressionType::None | CompressionType::Lzma => Ok(data.to_vec()),
        CompressionType::Lz4 => lz4_decompress(data),
        CompressionType::Zlib => zlib_decompress(data),
    }
}

pub fn optimize_layout(data: &[u8]) -> alloc::vec::Vec<u8> {
    let mut o = alloc::vec::Vec::with_capacity(data.len());
    for (i, c) in data.chunks(4096).enumerate() {
        if i % 2 == 0 {
            o.extend_from_slice(c);
        }
    }
    for (i, c) in data.chunks(4096).enumerate() {
        if i % 2 == 1 {
            o.extend_from_slice(c);
        }
    }
    o
}

fn lz4_compress(data: &[u8]) -> alloc::vec::Vec<u8> {
    let mut c = alloc::vec::Vec::new();
    let mut i = 0;
    while i < data.len() {
        if i + 4 < data.len()
            && data[i] == data[i + 1]
            && data[i] == data[i + 2]
            && data[i] == data[i + 3]
        {
            let r = data[i..].iter().take_while(|&&b| b == data[i]).count().min(127);
            c.push(0x80 | r as u8);
            c.push(data[i]);
            i += r;
        } else {
            c.push(data[i]);
            i += 1;
        }
    }
    c
}

fn lz4_decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>, &'static str> {
    let mut d = alloc::vec::Vec::new();
    let mut i = 0;
    while i < data.len() {
        if data[i] & 0x80 != 0 && i + 1 < data.len() {
            let r = (data[i] & 0x7F) as usize;
            for _ in 0..r {
                d.push(data[i + 1]);
            }
            i += 2;
        } else {
            d.push(data[i]);
            i += 1;
        }
    }
    Ok(d)
}

fn zlib_compress(data: &[u8]) -> alloc::vec::Vec<u8> {
    let mut r = alloc::vec::Vec::with_capacity(data.len() + 8);
    r.extend_from_slice(&[0x78, 0x9C]);
    r.extend_from_slice(data);
    r.extend_from_slice(
        &data.iter().fold(1u32, |a, &b| a.wrapping_add(u32::from(b))).to_be_bytes(),
    );
    r
}
fn zlib_decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>, &'static str> {
    if data.len() < 6 || data[0] != 0x78 {
        return Err("invalid zlib header");
    }
    Ok(data[2..data.len() - 4].to_vec())
}
