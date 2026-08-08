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

use crate::arch::cpu::get_cpu_id as cpu_id;
use crate::arch::read_time_counter as tsc_now;
use crate::crypto::sha3;
use crate::log::backend::{LogBackend, RamBufferBackend};
use crate::log::types::{LogEntry, Severity};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub static LOGGER: Mutex<Option<LogManager>> = Mutex::new(None);
pub static PANIC_MODE: AtomicBool = AtomicBool::new(false);

pub struct LogManager {
    backends: heapless::Vec<Box<dyn LogBackend>, 4>,
    ram_buffer: RamBufferBackend,
    last_hash: [u8; 32],
}

impl LogManager {
    pub const fn new() -> Self {
        Self {
            backends: heapless::Vec::new(),
            ram_buffer: RamBufferBackend::new(),
            last_hash: [0u8; 32],
        }
    }

    pub fn add_backend(&mut self, backend: Box<dyn LogBackend>) {
        let _ = self.backends.push(backend);
    }

    pub fn log(&mut self, sev: Severity, msg: &str) {
        let ts = tsc_now();
        let cpu = cpu_id();
        let mut entry =
            LogEntry { ts, cpu: cpu as u32, sev, msg: heapless::String::new(), hash: [0u8; 32] };
        let _ = entry.msg.push_str(msg);

        let mut hasher = sha3::Sha3_256::new();
        hasher.update(&self.last_hash);
        hasher.update(entry.msg.as_bytes());
        entry.hash.copy_from_slice(&hasher.finalize());
        self.last_hash = entry.hash;

        // The on-screen sinks (framebuffer/VGA console) carry only warnings and
        // worse, so a working desktop stays clean instead of scrolling boot and
        // subsystem debug chatter. The full stream, every severity, is still
        // recorded in the RAM buffer below for the in-system log viewer, so
        // nothing is lost. Lower this threshold to bring the trace back on screen.
        if (sev as u8) >= (Severity::Warn as u8) {
            for backend in self.backends.iter_mut() {
                backend.write(&entry);
            }
        }

        self.ram_buffer.write(&entry);
    }

    pub fn enter_panic_mode(&self) {
        PANIC_MODE.store(true, Ordering::SeqCst);
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.ram_buffer.get_entries()
    }

    pub fn get_recent(&self, count: usize) -> Vec<LogEntry> {
        self.ram_buffer.get_recent(count)
    }

    pub fn entry_count(&self) -> usize {
        self.ram_buffer.entry_count()
    }

    pub fn clear_buffer(&mut self) {
        self.ram_buffer.clear();
    }
}
