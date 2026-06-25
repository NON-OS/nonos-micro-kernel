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

pub const MAX_SOCKETS: usize = 32;

static TABLE: Mutex<[Option<(u32, SocketHandle)>; MAX_SOCKETS]> = Mutex::new([None; MAX_SOCKETS]);

pub fn alloc(owner_pid: u32, handle: SocketHandle) -> Option<u32> {
    let mut table = TABLE.lock();
    for (i, slot) in table.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some((owner_pid, handle));
            return Some(i as u32);
        }
    }
    None
}

pub fn get(index: u32, sender_pid: u32) -> Option<SocketHandle> {
    let table = TABLE.lock();
    table.get(index as usize).and_then(|s| {
        s.and_then(|(owner, h)| if owner == sender_pid { Some(h) } else { None })
    })
}

pub fn free(index: u32, sender_pid: u32) {
    let mut table = TABLE.lock();
    if let Some(slot) = table.get_mut(index as usize) {
        if matches!(slot, Some((owner, _)) if *owner == sender_pid) {
            *slot = None;
        }
    }
}
