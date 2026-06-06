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

pub(crate) const N_MK_IPC_SEND: i64 = tag4(b"MISD");
pub(crate) const N_MK_IPC_RECV: i64 = tag4(b"MIRC");
pub(crate) const N_MK_IPC_CALL: i64 = tag4(b"MICL");
pub(crate) const N_MK_IPC_RECV_FROM: i64 = tag4(b"MIRF");
pub(crate) const N_MK_IPC_REPLY: i64 = tag4(b"MIRY");
pub(crate) const N_MK_IPC_SEND_TO_PID: i64 = tag4(b"MISP");
pub(crate) const N_MK_SERVICE_LOOKUP: i64 = tag4(b"MSVL");
pub(crate) const N_MK_SERVICE_REGISTER: i64 = tag4(b"MSVR");
