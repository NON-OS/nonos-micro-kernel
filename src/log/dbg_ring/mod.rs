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

#[cfg(feature = "dbg-ring")]
mod drain;
#[cfg(feature = "dbg-ring")]
mod emit;
#[cfg(feature = "dbg-ring")]
mod storage;
#[cfg(feature = "dbg-ring")]
mod types;

#[cfg(feature = "dbg-ring")]
pub use drain::dbg_drain_to_serial;
#[cfg(feature = "dbg-ring")]
pub use emit::{dbg_emit, dbg_emit_2u64, dbg_emit_u64};

#[cfg(not(feature = "dbg-ring"))]
#[inline(always)]
pub fn dbg_emit(_tag: u32, _payload: &[u8]) {}

#[cfg(not(feature = "dbg-ring"))]
#[inline(always)]
pub fn dbg_emit_u64(_tag: u32, _v: u64) {}

#[cfg(not(feature = "dbg-ring"))]
#[inline(always)]
pub fn dbg_emit_2u64(_tag: u32, _a: u64, _b: u64) {}

#[cfg(not(feature = "dbg-ring"))]
#[inline(always)]
pub fn dbg_drain_to_serial() {}
