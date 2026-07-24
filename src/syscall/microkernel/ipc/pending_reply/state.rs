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
use alloc::string::String;
use spin::Mutex;

// Each pending entry is (caller reply inbox, correlation token). Carrying the
// caller's per-call token lets the redirect-reply path (a service replying via
// `mk_ipc_send` to its own fixed reply endpoint) stamp the reply with the
// correlation the caller waits on, so the caller can reject a forged reply
// (which can only ever carry correlation 0).
pub(super) static PENDING: Mutex<BTreeMap<u32, VecDeque<(String, u64)>>> =
    Mutex::new(BTreeMap::new());

pub(super) const MAX_PER_SERVICE: usize = 64;
