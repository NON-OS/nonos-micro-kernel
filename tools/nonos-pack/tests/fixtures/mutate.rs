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

use nonos_pack::container::decode;

fn trailer_start(sealed: &[u8]) -> usize {
    decode(sealed).expect("sealed package must decode").1
}

pub fn append_trailer_entry(sealed: &[u8], tag: u8, sig: &[u8]) -> Vec<u8> {
    let start = trailer_start(sealed);
    let mut out = sealed.to_vec();
    out[start] += 1;
    out.push(tag);
    out.extend_from_slice(&(sig.len() as u16).to_be_bytes());
    out.extend_from_slice(sig);
    out
}

pub fn corrupt_signature(sealed: &[u8], tag: u8) -> Vec<u8> {
    let start = trailer_start(sealed);
    let mut out = sealed.to_vec();
    let count = out[start] as usize;
    let mut p = start + 1;
    for _ in 0..count {
        let entry_tag = out[p];
        let len = u16::from_be_bytes([out[p + 1], out[p + 2]]) as usize;
        p += 3;
        if entry_tag == tag {
            out[p] ^= 0xFF;
            return out;
        }
        p += len;
    }
    panic!("sealed package has no trailer entry with tag {}", tag);
}
