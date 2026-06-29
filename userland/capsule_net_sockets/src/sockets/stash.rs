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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

static STASH: Mutex<BTreeMap<(u32, u32), Vec<u8>>> = Mutex::new(BTreeMap::new());

pub fn take(pid: u32, handle: u32, out: &mut [u8]) -> usize {
    let mut g = STASH.lock();
    let Some(buf) = g.get_mut(&(pid, handle)) else {
        return 0;
    };
    let n = core::cmp::min(out.len(), buf.len());
    out[..n].copy_from_slice(&buf[..n]);
    buf.drain(..n);
    if buf.is_empty() {
        g.remove(&(pid, handle));
    }
    n
}

pub fn put(pid: u32, handle: u32, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    STASH.lock().entry((pid, handle)).or_default().extend_from_slice(bytes);
}

pub fn has(pid: u32, handle: u32) -> bool {
    STASH.lock().get(&(pid, handle)).is_some_and(|b| !b.is_empty())
}
