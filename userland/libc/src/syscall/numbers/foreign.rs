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

//! Hosting code the kernel does not verify. Every one of these is gated on
//! the `ForeignExec` capability, which one capsule holds.

use super::tag::tag4;

pub(crate) const N_MK_FOREIGN_SPAWN: i64 = tag4(b"MFSP");
pub(crate) const N_MK_FOREIGN_START: i64 = tag4(b"MFST");
pub(crate) const N_MK_FOREIGN_WAIT: i64 = tag4(b"MFWT");
pub(crate) const N_MK_FOREIGN_REPLY: i64 = tag4(b"MFRP");
pub(crate) const N_MK_PEER_MAP: i64 = tag4(b"MPMP");
pub(crate) const N_MK_PEER_COPY: i64 = tag4(b"MPCP");
pub(crate) const N_MK_PEER_PROTECT: i64 = tag4(b"MPPT");
pub(crate) const N_MK_FOREIGN_THREAD: i64 = tag4(b"MFTH");
