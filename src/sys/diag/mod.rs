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

//! Trust-boundary diagnostics.
//!
//! A refusal at a capability gate is invisible from outside: the caller sees a
//! timeout and retries, and the system reads as slow rather than as denying.
//! Every gate that can refuse therefore reports here, with who asked, what
//! they held, and what the gate wanted.
//!
//! Healthy boots emit nothing from these sites, so the lines are signal, not
//! noise. Each site is rate limited on its own counter: the first `VERBOSE`
//! occurrences print, after that every `SAMPLE`th, so a capsule spinning on a
//! refused call cannot flood the console it is being diagnosed from.

use core::sync::atomic::{AtomicU32, Ordering};

use super::serial;

/// One instrumented gate. Declare as a `static` at the call site so each gate
/// rate-limits independently.
pub struct Site {
    label: &'static [u8],
    hits: AtomicU32,
}

const VERBOSE: u32 = 64;
const SAMPLE: u32 = 512;

impl Site {
    pub const fn new(label: &'static [u8]) -> Site {
        Site { label, hits: AtomicU32::new(0) }
    }

    fn admit(&self) -> Option<u32> {
        let n = self.hits.fetch_add(1, Ordering::Relaxed);
        (n < VERBOSE || n % SAMPLE == 0).then_some(n)
    }

    /// A capability gate refused `pid`: it held `have`, the gate wanted `need`.
    pub fn refused(&self, what: &str, pid: u32, have: u64, need: u64) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" refused ");
        serial::print_str(what);
        serial::print(b" pid=");
        serial::print_hex(pid as u64);
        serial::print(b" have=");
        serial::print_hex(have);
        serial::print(b" need=");
        serial::print_hex(need);
        trail(n);
    }

    /// The gate rejected `what` for a reason other than capabilities.
    pub fn reject(&self, what: &str, why: &'static str, pid: u32) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" ");
        serial::print_str(why);
        serial::print(b" ");
        serial::print_str(what);
        serial::print(b" pid=");
        serial::print_hex(pid as u64);
        trail(n);
    }

    /// A call that was sent and never answered: `errno` from the receive, the
    /// `server` that should have replied.
    pub fn starved(&self, what: &str, pid: u32, errno: i64, server: u32) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" unanswered ");
        serial::print_str(what);
        serial::print(b" caller=");
        serial::print_hex(pid as u64);
        serial::print(b" server=");
        serial::print_hex(server as u64);
        serial::print(b" err=");
        serial::print_hex(errno as u64);
        trail(n);
    }

    /// A state transition worth one line, `what` = `value`.
    pub fn note(&self, what: &str, value: u64) {
        let Some(n) = self.admit() else { return };
        serial::print(b"[DIAG] ");
        serial::print(self.label);
        serial::print(b" ");
        serial::print_str(what);
        serial::print(b"=");
        serial::print_hex(value);
        trail(n);
    }
}

fn trail(n: u32) {
    if n >= VERBOSE {
        serial::print(b" hits=");
        serial::print_hex(n as u64);
    }
    serial::println(b"");
}
