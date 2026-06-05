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

use alloc::vec::Vec;

use super::super::header::parse_response;
use super::super::wire::{NIC_HDR_LEN, OP_RX_PACKET};
use super::error::RxError;

pub(super) fn parse_payload(view: &[u8]) -> Result<Vec<u8>, RxError> {
    let (op, _, plen) = parse_response(view).ok_or(RxError::BadResponse)?;
    if op != OP_RX_PACKET {
        return Err(RxError::BadResponse);
    }
    let body = NIC_HDR_LEN + 4;
    if (plen as usize) < 4 || view.len() < body {
        return Err(RxError::BadResponse);
    }
    let status = i32::from_le_bytes([
        view[NIC_HDR_LEN],
        view[NIC_HDR_LEN + 1],
        view[NIC_HDR_LEN + 2],
        view[NIC_HDR_LEN + 3],
    ]);
    if status != 0 {
        return Err(RxError::Empty);
    }
    if (plen as usize) < 8 || view.len() < body + 4 {
        return Err(RxError::BadResponse);
    }
    let frame_len =
        u32::from_le_bytes([view[body], view[body + 1], view[body + 2], view[body + 3]]) as usize;
    let frame_start = body + 4;
    if frame_start + frame_len > view.len() {
        return Err(RxError::BadResponse);
    }
    Ok(view[frame_start..frame_start + frame_len].to_vec())
}
