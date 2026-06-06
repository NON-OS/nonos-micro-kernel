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
use crate::transport::wire::{le_u16, le_u32};

use super::constants::{HDR_LEN_V2, VERSION_V2};
use super::request_v2::RequestV2;

pub fn read_request_v2(bytes: &[u8]) -> Option<RequestV2> {
    if bytes.len() < HDR_LEN_V2 {
        return None;
    }
    let magic = le_u32(bytes, 0)?;
    let version = le_u16(bytes, 4)?;
    if version != VERSION_V2 {
        return None;
    }
    let op = le_u16(bytes, 6)?;
    let flags = le_u16(bytes, 8)?;
    let reply_port = le_u32(bytes, 12)?;
    let request_id = le_u32(bytes, 16)?;
    let payload_len = le_u32(bytes, 20)?;
    Some(RequestV2 { magic, op, flags, reply_port, request_id, payload_len })
}
