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

// Kernel-side mirror of `userland/capsule_vfs/src/protocol/*`.

use alloc::vec::Vec;

use crate::services::lifecycle::transport;

pub(super) const MAGIC: u32 = 0x4E4F_5646; // "NOVF"
pub(super) const VERSION: u16 = 1;

pub(super) const OP_OPEN: u16 = 1;
pub(super) const OP_CLOSE: u16 = 2;
pub(super) const OP_READ: u16 = 3;
pub(super) const OP_WRITE: u16 = 4;
pub(super) const OP_STAT: u16 = 5;
pub(super) const OP_LIST: u16 = 6;

pub(super) const O_CREATE: u32 = 1 << 0;
pub(super) const O_TRUNC: u32 = 1 << 1;
pub(super) const O_APPEND: u32 = 1 << 2;

// The path length is encoded as a single byte on the wire, so 255 is the
// largest value that round-trips. 256 would truncate to 0 (`256 as u8`) and
// desync the request from the path bytes that follow.
pub(super) const MAX_PATH_BYTES: u32 = 255;
pub(super) const MAX_DATA_BYTES: u32 = 65536;
pub(super) const MAX_PAYLOAD_BYTES: u32 = 65536;

pub(super) use crate::services::lifecycle::transport::DecodedResponse;

pub(super) fn encode_request(op: u16, flags: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    transport::encode_request(MAGIC, VERSION, op, flags, request_id, body)
}

pub(super) fn decode_response(buf: &[u8]) -> Option<DecodedResponse<'_>> {
    transport::decode_v1_response(buf, MAGIC, VERSION, MAX_PAYLOAD_BYTES)
}
