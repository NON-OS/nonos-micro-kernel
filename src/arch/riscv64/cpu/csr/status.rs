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

pub const SSTATUS_SIE: usize = 1 << 1;
pub const SSTATUS_SPIE: usize = 1 << 5;
pub const SSTATUS_SPP: usize = 1 << 8;
pub const SSTATUS_VS_SHIFT: usize = 9;
pub const SSTATUS_VS_MASK: usize = 0b11 << SSTATUS_VS_SHIFT;
pub const SSTATUS_VS_OFF: usize = 0 << SSTATUS_VS_SHIFT;
pub const SSTATUS_VS_INITIAL: usize = 1 << SSTATUS_VS_SHIFT;
pub const SSTATUS_VS_CLEAN: usize = 2 << SSTATUS_VS_SHIFT;
pub const SSTATUS_VS_DIRTY: usize = 3 << SSTATUS_VS_SHIFT;
pub const SSTATUS_FS_SHIFT: usize = 13;
pub const SSTATUS_FS_MASK: usize = 0b11 << SSTATUS_FS_SHIFT;
pub const SSTATUS_FS_OFF: usize = 0 << SSTATUS_FS_SHIFT;
pub const SSTATUS_FS_INITIAL: usize = 1 << SSTATUS_FS_SHIFT;
pub const SSTATUS_FS_CLEAN: usize = 2 << SSTATUS_FS_SHIFT;
pub const SSTATUS_FS_DIRTY: usize = 3 << SSTATUS_FS_SHIFT;
pub const SSTATUS_SUM: usize = 1 << 18;
pub const SSTATUS_MXR: usize = 1 << 19;
pub const SIE_SSIE: usize = 1 << 1;
pub const SIE_STIE: usize = 1 << 5;
pub const SIE_SEIE: usize = 1 << 9;
pub const SIP_SSIP: usize = 1 << 1;
pub const SIP_STIP: usize = 1 << 5;
pub const SIP_SEIP: usize = 1 << 9;
