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

pub const MAGIC: u32 = 0x4E59_4D31;
pub const HDR_LEN: usize = 20;
/// The mixnet capsule does network work between requests, a gateway
/// connection or a directory fetch, and answers once it is back. Sixty four
/// milliseconds was a local-call budget and reported a busy capsule as a dead
/// one.
pub const TIMEOUT_MS: u64 = 3_000;

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_TOPOLOGY_STATUS: u16 = 15;
pub const OP_TIMING_STATUS: u16 = 16;

/// Request header the nym server parses: magic, version 1, opcode,
/// then a zeroed errno/request-id/payload-len tail.
pub fn header(op: u16) -> [u8; HDR_LEN] {
    let mut tx = [0u8; HDR_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx
}
