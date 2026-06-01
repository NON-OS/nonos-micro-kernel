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

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

pub const N_IPC_SEND: i64 = tag4(b"MISD");
pub const N_IPC_RECV: i64 = tag4(b"MIRC");
pub const N_IPC_CALL: i64 = tag4(b"MICL");
pub const N_IPC_RECV_FROM: i64 = tag4(b"MIRF");
pub const N_IPC_REPLY: i64 = tag4(b"MIRY");
pub const N_IPC_SEND_TO_PID: i64 = tag4(b"MISP");
pub const N_SERVICE_LOOKUP: i64 = tag4(b"MSVL");
pub const N_SERVICE_REGISTER: i64 = tag4(b"MSVR");
pub const N_MMAP: i64 = tag4(b"MMAP");
pub const N_MUNMAP: i64 = tag4(b"MUMP");
pub const N_EXIT: i64 = tag4(b"MEXT");
pub const N_YIELD: i64 = tag4(b"MYLD");
pub const N_TIME_MILLIS: i64 = tag4(b"MTMS");
pub const N_CRYPTO_RANDOM: i64 = tag4(b"CRND");
pub const N_DEBUG: i64 = tag4(b"MDBG");
pub const N_DISPLAY_DIMENSIONS: i64 = tag4(b"GDIM");
pub const N_SURFACE_REGISTER: i64 = tag4(b"MSRG");
pub const N_SURFACE_SHARE: i64 = tag4(b"MSSH");
pub const N_SURFACE_ATTACH: i64 = tag4(b"MSAT");
pub const N_SURFACE_RELEASE: i64 = tag4(b"MSRL");
pub const N_SURFACE_PRESENT: i64 = tag4(b"MSPR");
pub const N_INPUT_EVENT_DRAIN: i64 = tag4(b"MIED");
