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

use super::types::CapsuleState;

impl CapsuleState {
    pub fn restart_count(&self) -> u32 {
        self.restart_count.load(Ordering::SeqCst)
    }
    pub fn last_exit_ms(&self) -> u64 {
        self.last_exit_ms.load(Ordering::SeqCst)
    }
    pub fn record_exit(&self, now_ms: u64) {
        self.last_exit_ms.store(now_ms, Ordering::SeqCst);
        self.restart_count.fetch_add(1, Ordering::SeqCst);
    }
    pub fn should_respawn(&self, now_ms: u64) -> bool {
        if self.restart_count() >= self.max_restarts.load(Ordering::SeqCst) {
            return false;
        }
        let last = self.last_exit_ms();
        last == 0 || now_ms.saturating_sub(last) >= self.debounce_ms.load(Ordering::SeqCst)
    }
    pub fn set_max_restarts(&self, value: u32) {
        self.max_restarts.store(value, Ordering::SeqCst);
    }
    pub fn set_debounce_ms(&self, value: u64) {
        self.debounce_ms.store(value, Ordering::SeqCst);
    }
}
