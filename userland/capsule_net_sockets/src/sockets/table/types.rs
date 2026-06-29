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

use core::sync::atomic::AtomicU32;
use spin::Mutex;

use crate::sockets::{Kind, LocalAddr4, RemoteAddr4, SocketKey};

pub const TABLE_CAP: usize = 256;

#[derive(Clone, Copy)]
pub struct Socket {
    pub key: SocketKey,
    pub kind: Kind,
    pub local: Option<LocalAddr4>,
    pub remote: Option<RemoteAddr4>,
    pub transport_handle: u32,
    pub bound: bool,
    pub listening: bool,
    pub nonblock: bool,
    pub timeout_ms: u32,
}

pub struct Table {
    pub(super) inner: Mutex<[Option<Socket>; TABLE_CAP]>,
    pub(super) next_handle: AtomicU32,
}

impl Table {
    pub const fn new() -> Self {
        Self { inner: Mutex::new([None; TABLE_CAP]), next_handle: AtomicU32::new(1) }
    }
}

pub static SOCKETS: Table = Table::new();
