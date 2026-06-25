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

use spin::Mutex;
use smoltcp::iface::SocketHandle;

pub const MAX_UDP_SOCKETS: usize = 16;

struct Entry {
    owner_pid: u32,
    local_port: u16,
    handle: SocketHandle,
}

static TABLE: Mutex<[Option<Entry>; MAX_UDP_SOCKETS]> =
    Mutex::new([None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None]);

pub fn insert(owner_pid: u32, local_port: u16, handle: SocketHandle) -> bool {
    let mut table = TABLE.lock();
    for slot in table.iter_mut() {
        if slot.is_none() {
            *slot = Some(Entry { owner_pid, local_port, handle });
            return true;
        }
    }
    false
}

pub fn get(owner_pid: u32, local_port: u16) -> Option<SocketHandle> {
    let table = TABLE.lock();
    table.iter().find_map(|s| {
        s.as_ref().and_then(|e| {
            if e.owner_pid == owner_pid && e.local_port == local_port {
                Some(e.handle)
            } else {
                None
            }
        })
    })
}

pub fn remove(owner_pid: u32, local_port: u16) {
    let mut table = TABLE.lock();
    for slot in table.iter_mut() {
        let matches = slot.as_ref().map_or(false, |e| {
            e.owner_pid == owner_pid && e.local_port == local_port
        });
        if matches {
            *slot = None;
            return;
        }
    }
}
