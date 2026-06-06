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

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use spin::Mutex;

const MAX_PENDING_PER_PID: usize = 16;

static PENDING: Mutex<BTreeMap<u32, VecDeque<u64>>> = Mutex::new(BTreeMap::new());

pub(super) fn note_request(client_pid: u32, token: u64) {
    if token == 0 {
        return;
    }
    let mut map = PENDING.lock();
    let queue = map.entry(client_pid).or_insert_with(VecDeque::new);
    if queue.len() >= MAX_PENDING_PER_PID {
        queue.pop_front();
    }
    queue.push_back(token);
}

pub(super) fn take_for_reply(client_pid: u32) -> u64 {
    let mut map = PENDING.lock();
    match map.get_mut(&client_pid) {
        Some(queue) => queue.pop_front().unwrap_or(0),
        None => 0,
    }
}
