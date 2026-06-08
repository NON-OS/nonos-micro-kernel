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
use super::tag::tag4;

pub(crate) const N_MK_MMAP: i64 = tag4(b"MMAP");
pub(crate) const N_MK_EXIT: i64 = tag4(b"MEXT");
pub(crate) const N_MK_PID_ALIVE: i64 = tag4(b"MPAL");
pub(crate) const N_MK_YIELD: i64 = tag4(b"MYLD");
pub(crate) const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
pub(crate) const N_MK_TIME_RTC: i64 = tag4(b"MTRT");
pub(crate) const N_MK_BATTERY_STATUS: i64 = tag4(b"MBAT");
pub(crate) const N_MK_ATTEST_STATUS: i64 = tag4(b"MAST");
