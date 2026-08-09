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

// A store that fails to decode at boot used to become a silently empty
// /capsules. The first failure's code is kept here so OP_STORE_STATUS can
// report it; later failures never overwrite the original evidence.
use core::sync::atomic::{AtomicU32, Ordering};

use super::error::BlkError;

static STORE_STATUS: AtomicU32 = AtomicU32::new(0);

pub fn record(err: &BlkError) {
    let _ = STORE_STATUS.compare_exchange(0, code(err), Ordering::Relaxed, Ordering::Relaxed);
}

pub fn current() -> u32 {
    STORE_STATUS.load(Ordering::Relaxed)
}

fn code(err: &BlkError) -> u32 {
    match err {
        BlkError::NoService => 1,
        BlkError::Transport(_) => 2,
        BlkError::ShortReply(_) => 3,
        BlkError::BadHeader => 4,
        BlkError::IdMismatch => 5,
        BlkError::BadLength => 6,
        BlkError::Status(_) => 7,
        BlkError::Inval => 8,
        BlkError::BadContainer => 9,
        BlkError::Exists => 10,
    }
}
