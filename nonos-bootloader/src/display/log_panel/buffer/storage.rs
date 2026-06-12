// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::timer::boot_timestamp;
use crate::display::log_panel::entry::LogEntry;
use crate::display::log_panel::types::{LogLevel, MAX_LOG_LINES};
use core::sync::atomic::{AtomicUsize, Ordering};

static mut LOG_BUFFER: [LogEntry; MAX_LOG_LINES] = [LogEntry::empty(); MAX_LOG_LINES];
static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

pub fn push_entry(level: LogLevel, msg: &[u8]) -> usize {
    let count = LOG_COUNT.load(Ordering::Relaxed);
    let idx = count % MAX_LOG_LINES;
    let ts = boot_timestamp();
    unsafe {
        LOG_BUFFER[idx].set(level, msg, ts);
    }
    LOG_COUNT.store(count + 1, Ordering::Release);
    count + 1
}

pub fn get_count() -> usize {
    LOG_COUNT.load(Ordering::Relaxed)
}

pub fn get_entry(idx: usize) -> Option<LogEntry> {
    if idx >= MAX_LOG_LINES {
        return None;
    }
    unsafe { Some(LOG_BUFFER[idx]) }
}

pub fn clear_buffer() {
    LOG_COUNT.store(0, Ordering::Release);
}
