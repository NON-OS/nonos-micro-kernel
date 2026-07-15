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

pub const ATA_IDENTIFY: u8 = 0xec;
pub const ATA_READ_DMA_EXT: u8 = 0x25;
pub const ATA_WRITE_DMA_EXT: u8 = 0x35;
pub const ATA_FLUSH_EXT: u8 = 0xea;

pub const FIS_TYPE_REG_H2D: u8 = 0x27;
pub const FIS_H2D_COMMAND: u8 = 1 << 7;
pub const FIS_H2D_LEN_DWORDS: u8 = 5;
pub const ATA_DEV_LBA: u8 = 1 << 6;

pub const CMD_HEADER_WRITE: u8 = 1 << 6;

pub const SECTOR_SIZE: usize = 512;
pub const MAX_SECTORS: u32 = 64;
pub const DATA_BUF_BYTES: u64 = MAX_SECTORS as u64 * SECTOR_SIZE as u64;
pub const STRUCT_REGION_BYTES: u64 = 4096;
pub const COMPLETION_POLL_LIMIT: u32 = 5_000_000;
/// Milliseconds to hold the PHY in COMRESET before releasing it.
pub const COMRESET_HOLD_MS: u64 = 2;
/// Wall-time bound on waiting for the PHY link and the device to become ready.
/// A real SATA drive can take hundreds of milliseconds to link up after spin-up;
/// this is measured against the monotonic clock, not a CPU-speed-dependent spin
/// count.
pub const LINK_TIMEOUT_MS: u64 = 2_000;
