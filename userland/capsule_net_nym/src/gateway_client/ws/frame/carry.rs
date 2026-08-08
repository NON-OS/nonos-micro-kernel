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

use alloc::vec::Vec;
use spin::Mutex;

/// Bytes read off the socket that no completed frame claimed.
///
/// A gateway pushes messages in bursts, so one read can hand back several
/// frames at once, and a frame can also straddle two reads. Both cases need
/// somewhere for the remainder to live between calls. Without it the second
/// frame in a segment is lost, and lost silently: the socket has nothing more
/// to give, so the loss looks exactly like an idle link.
static CARRY: Mutex<(u32, Vec<u8>)> = Mutex::new((0, Vec::new()));

/// Take what the last read left over for this stream.
///
/// A different stream means the old connection is gone, and its leftovers
/// belong to a session whose keys no longer exist.
pub fn take(stream: u32) -> Vec<u8> {
    let mut held = CARRY.lock();
    if held.0 != stream {
        held.0 = stream;
        held.1 = Vec::new();
    }
    core::mem::take(&mut held.1)
}

pub fn keep(stream: u32, rest: &[u8]) {
    let mut held = CARRY.lock();
    held.0 = stream;
    held.1.clear();
    held.1.extend_from_slice(rest);
}
