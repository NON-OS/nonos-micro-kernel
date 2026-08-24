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

//! Entropy, the IPC secret, the boot CPU, the scheduler and the clocks.
//! Everything after this stage assumes all five, so each failure is fatal
//! here rather than surfacing later as a swallowed error somewhere else.

use super::fatal::fatal;
use crate::boot::handoff::KernelHandoff;
use crate::sys::clock;

pub(super) fn init_core_services(handoff: &KernelHandoff) {
    crate::sys::policy::hostname_init();
    if crate::crypto::util::rng::init_rng().is_err() {
        fatal("crypto: init_rng failed", "entropy unavailable");
    }
    if let Err(e) = crate::ipc::nonos_channel::init_ipc_secret() {
        fatal("ipc: init_ipc_secret failed", e);
    }
    if let Err(e) = crate::smp::init_bsp() {
        fatal("smp: init_bsp failed", e);
    }
    crate::sched::init();

    let freq = handoff.timing.fixed_freq_hz.unwrap_or(0);
    clock::init(freq, handoff.timing.unix_epoch_ms);
    // The wall clock and the uptime counter hold their own copies of the
    // counter frequency. Seeding only the first leaves uptime reporting zero
    // forever, and every wait on elapsed time then never finishes. The x86_64
    // binary seeds the second from `init_core_systems`, which no other entry
    // path runs, so it belongs here where every arch passes through.
    crate::sys::timer::tsc::init(freq, handoff.timing.unix_epoch_ms);
}
