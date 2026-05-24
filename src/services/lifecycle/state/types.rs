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

use core::sync::atomic::{AtomicU32, AtomicU64};

use super::constants::{DEFAULT_MAX_RESTARTS, DEFAULT_RESPAWN_DEBOUNCE_MS};

pub struct CapsuleState {
    pub(super) pid: AtomicU32,
    pub(super) generation: AtomicU64,
    pub(super) restart_count: AtomicU32,
    pub(super) last_exit_ms: AtomicU64,
    pub(super) max_restarts: AtomicU32,
    pub(super) debounce_ms: AtomicU64,
}

impl CapsuleState {
    pub const fn new() -> Self {
        Self {
            pid: AtomicU32::new(0),
            generation: AtomicU64::new(0),
            restart_count: AtomicU32::new(0),
            last_exit_ms: AtomicU64::new(0),
            max_restarts: AtomicU32::new(DEFAULT_MAX_RESTARTS),
            debounce_ms: AtomicU64::new(DEFAULT_RESPAWN_DEBOUNCE_MS),
        }
    }
}
