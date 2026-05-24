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

pub const PRIORITY_BASE: u64 = 0x0000;
pub const PENDING_BASE: u64 = 0x1000;
pub const ENABLE_BASE: u64 = 0x2000;
pub const THRESHOLD_BASE: u64 = 0x20_0000;
pub const CLAIM_BASE: u64 = 0x20_0004;
pub const ENABLE_STRIDE: u64 = 0x80;
pub const CONTEXT_STRIDE: u64 = 0x1000;
pub const MAX_INTERRUPTS: u32 = 1024;
pub const DEFAULT_BASE: u64 = 0x0C00_0000;
