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

use core::sync::atomic::Ordering;

use crate::sockets::{Kind, SocketKey};

use super::types::{Socket, Table};

impl Table {
    pub fn open(&self, pid: u32, kind: Kind) -> Option<SocketKey> {
        let handle = self.next_handle.fetch_add(1, Ordering::Relaxed);
        let key = SocketKey { pid, handle };
        let mut g = self.inner.lock();
        for slot in g.iter_mut() {
            if slot.is_none() {
                *slot = Some(Socket::new(key, kind));
                return Some(key);
            }
        }
        None
    }
}

impl Socket {
    pub fn new(key: SocketKey, kind: Kind) -> Self {
        Self {
            key,
            kind,
            local: None,
            remote: None,
            transport_handle: 0,
            bound: false,
            listening: false,
            nonblock: false,
            timeout_ms: 0,
        }
    }
}
