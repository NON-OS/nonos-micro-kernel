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

use crate::protocol::{read_u32, E_INVAL};

pub const DECODE_HDR_LEN: usize = 16;

#[derive(Clone, Copy)]
pub enum DecodeKind {
    Png,
    Bmp,
    Lz4Raw,
    Jpeg,
}

pub struct DecodeReq<'a> {
    pub kind: DecodeKind,
    pub width: u32,
    pub height: u32,
    pub payload: &'a [u8],
}

pub fn parse_decode_req(body: &[u8]) -> Result<DecodeReq<'_>, i32> {
    if body.len() < DECODE_HDR_LEN {
        return Err(E_INVAL);
    }
    let Some(kind_raw) = read_u32(body, 0) else {
        return Err(E_INVAL);
    };
    let Some(width) = read_u32(body, 4) else {
        return Err(E_INVAL);
    };
    let Some(height) = read_u32(body, 8) else {
        return Err(E_INVAL);
    };
    let Some(payload_len) = read_u32(body, 12).map(|v| v as usize) else {
        return Err(E_INVAL);
    };
    if body.len() != DECODE_HDR_LEN + payload_len {
        return Err(E_INVAL);
    }
    let kind = match kind_raw {
        1 => DecodeKind::Png,
        2 => DecodeKind::Bmp,
        3 => DecodeKind::Lz4Raw,
        4 => DecodeKind::Jpeg,
        _ => return Err(E_INVAL),
    };
    Ok(DecodeReq { kind, width, height, payload: &body[DECODE_HDR_LEN..] })
}
