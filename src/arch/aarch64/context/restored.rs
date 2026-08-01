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

//! The per-CPU "this return is a resume" flag.
//!
//! A saved context returns twice: once when it is taken, once when it is put
//! back. The two returns are identical from the calling code's side, so the
//! restore path raises this flag and the caller reads it to tell them apart. It
//! is per-CPU because two cores can be mid-switch at the same moment and one
//! must not consume the other's answer.

use core::sync::atomic::AtomicBool;

const MAX_CPUS: usize = 256;
const INIT_FALSE: AtomicBool = AtomicBool::new(false);

pub(super) static CONTEXT_JUST_RESTORED: [AtomicBool; MAX_CPUS] = [INIT_FALSE; MAX_CPUS];

/// This CPU's slot. Wrapped rather than asserted: a CPU id past the table is a
/// bring-up bug, and sharing a slot degrades the switch rather than faulting in
/// the middle of one.
#[inline]
pub(super) fn current_cpu_index() -> usize {
    (crate::process::scheduler::api::current_cpu_id() as usize) % MAX_CPUS
}

/// Mark the current CPU as having just come back from a saved context.
pub fn set_restored_flag() {
    CONTEXT_JUST_RESTORED[current_cpu_index()].store(true, core::sync::atomic::Ordering::SeqCst);
}
