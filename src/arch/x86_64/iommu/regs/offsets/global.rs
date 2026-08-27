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

pub const VER: usize = 0x000;
pub const CAP: usize = 0x008;
pub const ECAP: usize = 0x010;
pub const GCMD: usize = 0x018;
pub const GSTS: usize = 0x01C;
pub const RTADDR: usize = 0x020;
pub const FSTS: usize = 0x034;
pub const FECTL: usize = 0x038;

/// GCMD is write-only, not read-modify-write: every write sets one command and
/// must carry the state of the others, which is what GSTS is read for.
pub const GCMD_TE: u32 = 1 << 31;
pub const GCMD_SRTP: u32 = 1 << 30;
pub const GCMD_WBF: u32 = 1 << 27;

pub const GSTS_TES: u32 = 1 << 31;
pub const GSTS_RTPS: u32 = 1 << 30;
pub const GSTS_WBFS: u32 = 1 << 27;
