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

//! Wire-form constants the kernel client and the userland capsule
//! both speak. Drift between the two manifests as
//! `DriverBlkError::ProtocolMismatch`. The userland mirror lives
//! at `userland/capsule_driver_virtio_blk/src/protocol/header.rs`
//! and `userland/capsule_driver_virtio_blk/src/constants/queue.rs`.

pub(in super::super) const MAGIC: u32 = 0x4E42_4C4B; // "NBLK"
pub(in super::super) const VERSION: u16 = 1;

pub(in super::super) const SECTOR_SIZE: usize = 512;

// Per-request ceiling, matches `MAX_SECTORS_PER_REQUEST` in the
// userland capsule (64 sectors → 32 KiB).
pub(in super::super) const MAX_SECTORS_PER_REQUEST: u32 = 64;
pub(in super::super) const MAX_RW_PAYLOAD_BYTES: u32 = MAX_SECTORS_PER_REQUEST * SECTOR_SIZE as u32;

// Response cap. Read replies carry up to MAX_RW_PAYLOAD_BYTES of
// data after a 4-byte status; that bound plus header slack is
// what the transport accepts.
pub(in super::super) const MAX_PAYLOAD_BYTES: u32 = MAX_RW_PAYLOAD_BYTES + 64;
