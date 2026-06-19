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

pub const ECHO_REQUEST: u8 = 8;
pub const ECHO_REPLY: u8 = 0;
const HDR: usize = 8;

pub fn build_echo(id: u16, seq: u16, payload: &[u8], out: &mut [u8]) -> Option<usize> {
    let total = HDR + payload.len();
    if out.len() < total {
        return None;
    }
    out[0] = ECHO_REQUEST;
    out[1] = 0;
    out[2] = 0;
    out[3] = 0;
    out[4..6].copy_from_slice(&id.to_be_bytes());
    out[6..8].copy_from_slice(&seq.to_be_bytes());
    out[HDR..total].copy_from_slice(payload);
    let ck = checksum(&out[..total]);
    out[2..4].copy_from_slice(&ck.to_be_bytes());
    Some(total)
}

pub fn parse_reply(msg: &[u8]) -> Option<(u8, u16, u16)> {
    if msg.len() < HDR {
        return None;
    }
    let id = u16::from_be_bytes([msg[4], msg[5]]);
    let seq = u16::from_be_bytes([msg[6], msg[7]]);
    Some((msg[0], id, seq))
}

fn checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i + 1]]) as u32;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}
