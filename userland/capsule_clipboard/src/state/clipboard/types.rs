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

use alloc::collections::VecDeque;

use crate::state::Entry;

pub struct Clipboard {
    pub(super) items: VecDeque<Entry>,
    pub(super) total_bytes: usize,
    pub(super) max_depth: usize,
    pub(super) max_total_bytes: usize,
    pub(super) last_activity_ms: u64,
    pub(super) idle_timeout_ms: u64,
}

impl Clipboard {
    pub fn new(
        max_depth: usize,
        max_total_bytes: usize,
        idle_timeout_ms: u64,
        now_ms: u64,
    ) -> Self {
        Self {
            items: VecDeque::new(),
            total_bytes: 0,
            max_depth,
            max_total_bytes,
            last_activity_ms: now_ms,
            idle_timeout_ms,
        }
    }
}
