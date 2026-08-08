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

use super::state::{LogManager, LOGGER, PANIC_MODE};
#[cfg(target_arch = "x86_64")]
use crate::log::backend::VgaBackend;
use crate::log::types::{LogEntry, Severity};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use spin::Mutex;

pub fn init() {
    let mut l = LOGGER.lock();
    let mut mgr = LogManager::new();
    #[cfg(target_arch = "x86_64")]
    mgr.add_backend(Box::new(VgaBackend));
    *l = Some(mgr);
}

pub fn log(sev: Severity, msg: &str) {
    if let Some(mgr) = LOGGER.lock().as_mut() {
        mgr.log(sev, msg);
    }
}

pub fn enter_panic_mode() {
    PANIC_MODE.store(true, Ordering::SeqCst);
}

pub fn log_critical(msg: &str) {
    log(Severity::Fatal, msg);
}

pub fn try_get_logger() -> Option<&'static Mutex<Option<LogManager>>> {
    Some(&LOGGER)
}

pub fn get_log_entries() -> Vec<LogEntry> {
    if let Some(mgr) = LOGGER.lock().as_ref() {
        mgr.get_entries()
    } else {
        Vec::new()
    }
}

pub fn get_recent_logs(count: usize) -> Vec<LogEntry> {
    if let Some(mgr) = LOGGER.lock().as_ref() {
        mgr.get_recent(count)
    } else {
        Vec::new()
    }
}

pub fn log_entry_count() -> usize {
    if let Some(mgr) = LOGGER.lock().as_ref() {
        mgr.entry_count()
    } else {
        0
    }
}

pub fn clear_log_buffer() {
    if let Some(mgr) = LOGGER.lock().as_mut() {
        mgr.clear_buffer();
    }
}
