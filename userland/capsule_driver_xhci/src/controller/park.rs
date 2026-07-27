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
use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::{mk_irq_wait, mk_yield};

// Event-ring polls burned before the first park. The controller writes the
// completion TRB while the doorbell store is still in flight, so a device that
// answers never reaches the budget and never pays a syscall.
pub const SPIN_BUDGET: u32 = 1024;
// Ceiling on a single park. The controller interrupt wakes the capsule the
// moment the event lands, so this only bounds the path where no interrupt
// arrives at all; a device that never answers then costs one wakeup per
// scheduler tick instead of a whole core.
const PARK_MS: u64 = 2;

static IRQ_GRANT: AtomicU64 = AtomicU64::new(0);
static IRQ_SEQ: AtomicU64 = AtomicU64::new(0);

pub fn set_irq_grant(grant_id: u64) {
    IRQ_GRANT.store(grant_id, Ordering::Relaxed);
}

// One step of a completion poll: spin while the budget lasts, then block off
// the run queue until the controller interrupt or the park ceiling.
pub fn park_step(spins: u32) {
    if spins <= SPIN_BUDGET {
        core::hint::spin_loop();
        return;
    }
    let grant = IRQ_GRANT.load(Ordering::Relaxed);
    if grant == 0 {
        mk_yield();
        return;
    }
    let mut seq = IRQ_SEQ.load(Ordering::Relaxed);
    if mk_irq_wait(grant, seq, PARK_MS, &mut seq) < 0 {
        IRQ_GRANT.store(0, Ordering::Relaxed);
        mk_yield();
        return;
    }
    IRQ_SEQ.store(seq, Ordering::Relaxed);
}
