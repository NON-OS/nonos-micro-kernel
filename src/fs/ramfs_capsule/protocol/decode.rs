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

use crate::services::lifecycle::transport::DecodedResponse;

const RESPONSE_HDR_LEN: usize = 8;

pub(super) fn decode_response(buf: &[u8]) -> Option<DecodedResponse<'_>> {
    if buf.len() < RESPONSE_HDR_LEN {
        return None;
    }
    let request_id = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let status = i32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
    Some(DecodedResponse { op: 0, request_id, status, body: &buf[RESPONSE_HDR_LEN..] })
}
