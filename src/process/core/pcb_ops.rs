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

use super::pcb::ProcessControlBlock;
use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::capabilities::CapabilityToken;

impl ProcessControlBlock {
    pub fn capability_token(&self) -> crate::syscall::capabilities::CapabilityToken {
        (**self.capability_token.read()).clone()
    }

    /// Resolver-facing handle. Avoids the inner `Vec` clone that
    /// `capability_token()` pays per call. Hot path for syscall
    /// dispatch and the future MAC/zeroization-aware checks.
    pub fn capability_token_arc(&self) -> Arc<CapabilityToken> {
        Arc::clone(&self.capability_token.read())
    }

    #[inline]
    pub fn revocation_epoch(&self) -> u64 {
        self.revocation_epoch.load(Ordering::Acquire)
    }

    pub fn set_alarm(&self, seconds: u32) -> u32 {
        let now_ms = crate::time::timestamp_millis();
        let old_alarm_ms = self.alarm_time_ms.load(Ordering::Acquire);
        let remaining =
            if old_alarm_ms > now_ms { ((old_alarm_ms - now_ms) / 1000) as u32 } else { 0 };
        let new_alarm_ms =
            if seconds == 0 { 0 } else { now_ms.saturating_add((seconds as u64) * 1000) };
        self.alarm_time_ms.store(new_alarm_ms, Ordering::Release);
        remaining
    }

    pub fn check_alarm_expired(&self) -> bool {
        let alarm_ms = self.alarm_time_ms.load(Ordering::Acquire);
        if alarm_ms == 0 {
            return false;
        }
        let now_ms = crate::time::timestamp_millis();
        if now_ms >= alarm_ms {
            self.alarm_time_ms.store(0, Ordering::Release);
            true
        } else {
            false
        }
    }

    pub fn on_thread_exit(&self) {
        let clear_tid_ptr = self.clear_child_tid.load(Ordering::Acquire);
        if clear_tid_ptr != 0 {
            let _ = crate::usercopy::write_user_value::<u32>(clear_tid_ptr, &0);
        }
        if let Some(ref tg) = self.thread_group {
            tg.remove_thread(self.pid);
        }
    }

    pub fn reply_inbox(&self) -> Option<&'static str> {
        *self.reply_inbox.read()
    }

    pub fn set_reply_inbox(&self, inbox: &'static str) {
        *self.reply_inbox.write() = Some(inbox);
    }
}
