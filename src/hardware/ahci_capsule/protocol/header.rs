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

pub(in super::super) const MAGIC: u32 = 0x4E41_4843;
pub(in super::super) const VERSION: u16 = 1;

pub(in super::super) const CONTROLLER_INFO_PAYLOAD_LEN: usize = 24;
pub(in super::super) const PORT_LIST_HEADER_BYTES: usize = 4;
pub(in super::super) const PORT_ENTRY_BYTES: usize = 36;
pub(in super::super) const MAX_PORTS: usize = 32;
pub(in super::super) const SECTOR_SIZE: usize = 512;
pub(in super::super) const MAX_SECTORS: u32 = 64;
pub(in super::super) const MAX_RW_PAYLOAD_BYTES: u32 = MAX_SECTORS * SECTOR_SIZE as u32;

const PORT_LIST_PAYLOAD_LEN: usize = PORT_LIST_HEADER_BYTES + MAX_PORTS * PORT_ENTRY_BYTES;
const _: () = assert!(MAX_RW_PAYLOAD_BYTES as usize > PORT_LIST_PAYLOAD_LEN);
pub(in super::super) const MAX_PAYLOAD_BYTES: u32 = MAX_RW_PAYLOAD_BYTES;
