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

//! What this CPU's local APIC says about itself, read back from the part.
//!
//! Arming a timer is a write, and a write tells you what was asked for. On the
//! boot CPU the difference never mattered, because a timer that did not arm
//! showed up immediately as a dead scheduler. An application processor has no
//! such tell: it parks in its idle loop, and a timer that was never armed and
//! a timer that is ticking into a handler that does nothing look identical
//! from outside. So each AP reads its own registers back and says what it
//! found.

use core::sync::atomic::Ordering;

use super::constants::{
    LAPIC_ISR_BASE, LAPIC_ISR_WORDS, LAPIC_LVT_TIMER, LAPIC_PPR, LAPIC_SVR, LAPIC_TIMER_CURRENT,
    LAPIC_TIMER_INIT,
};
use super::regs::lapic_read_raw;
use super::state::LAPIC_INIT;
use crate::sys::serial;

/// The lowest vector this CPU has in service, if any.
///
/// A vector stays in service from delivery until its handler writes EOI, and
/// the local APIC will not deliver anything of equal or lower priority while
/// it is set. So a handler that dies before acknowledging does not merely lose
/// one interrupt, it silences that priority class for the life of the CPU,
/// while leaving higher classes working. A CPU that takes inter-processor
/// interrupts at 0x40 and no timer at 0x20 looks exactly like that.
fn in_service() -> Option<u32> {
    for word in 0..LAPIC_ISR_WORDS {
        // SAFETY: eK@nonos.systems - reads of this CPU's own in-service
        // register, which has no side effects. EOI is the only write that
        // changes it and nothing here writes.
        let bits = unsafe { lapic_read_raw(LAPIC_ISR_BASE + word * 0x10) };
        if bits != 0 {
            return Some(word * 32 + bits.trailing_zeros());
        }
    }
    None
}

/// The current-count register sampled twice, far enough apart that a running
/// timer must have moved. Equal samples mean the counter is not advancing,
/// which is the difference between "armed" and "armed and running".
///
/// The gap between samples is deliberately small. `spin_loop` emits `pause`,
/// which costs on the order of a hundred cycles on current parts, so a spin of
/// a few hundred thousand iterations is tens of milliseconds. An earlier
/// version used two hundred thousand and so held every application processor
/// inside this function for longer than `SHOOTDOWN_TIMEOUT_TSC`, which is the
/// deadline this report exists to explain. A diagnostic that runs longer than
/// the timeout it is diagnosing measures itself.
///
/// The timer's divider makes a few thousand cycles plenty: the count moves
/// once per sixteen bus cycles and any decrement at all answers the question.
fn counter_moves() -> bool {
    // SAFETY: eK@nonos.systems - a read of this CPU's own LAPIC current-count
    // register, which has no side effects.
    let first = unsafe { lapic_read_raw(LAPIC_TIMER_CURRENT) };
    for _ in 0..2_000 {
        core::hint::spin_loop();
    }
    // SAFETY: eK@nonos.systems - as above.
    let second = unsafe { lapic_read_raw(LAPIC_TIMER_CURRENT) };
    first != second
}

/// Print this CPU's timer state. Called from an AP once its own LAPIC has been
/// programmed, before it enters the idle loop.
pub fn report_local_timer(cpu_id: u32) {
    if !LAPIC_INIT.load(Ordering::Relaxed) {
        let mut l = serial::Line::new();
        l.str(b"[SMP-AP] cpu=").dec(cpu_id as u64).str(b" lapic not initialised");
        l.end();
        return;
    }
    // SAFETY: eK@nonos.systems - reads of this CPU's own LAPIC registers.
    let (svr, lvt, init) = unsafe {
        (
            lapic_read_raw(LAPIC_SVR),
            lapic_read_raw(LAPIC_LVT_TIMER),
            lapic_read_raw(LAPIC_TIMER_INIT),
        )
    };
    // Sampled before the line is built: `counter_moves` spins, and spinning
    // with a half-built line is how this report lost the field it exists for.
    let counting = counter_moves();
    let isr = in_service();

    /*
     * Recorded before the line is built. Printing takes the serial lock, and a
     * CPU that sits on it still has to be able to say what its APIC was doing:
     * these registers are per-CPU at a single address, so no other CPU can
     * read them on its behalf.
     */
    if let Some(cpu) = crate::smp::get_cpu(cpu_id as usize) {
        // 1 for "looked, found nothing", vector + 2 otherwise. 0 stays
        // reserved for "has not looked", which is what a CPU that never
        // reached here still reports.
        cpu.in_service_seen.store(isr.map_or(1, |v| v + 2), Ordering::Relaxed);
    }

    let mut l = serial::Line::new();
    l.str(b"[SMP-AP] cpu=").dec(cpu_id as u64);
    l.str(b" svr=").hex(svr as u64);
    l.str(b" lvt_timer=").hex(lvt as u64);
    l.str(b" init=").dec(init as u64);
    l.str(b" counting=").dec(u64::from(counting));
    // SAFETY: eK@nonos.systems - a read of this CPU's own priority register.
    l.str(b" ppr=").hex(unsafe { lapic_read_raw(LAPIC_PPR) } as u64);
    match isr {
        Some(v) => l.str(b" in_service=").hex(v as u64),
        None => l.str(b" in_service=none"),
    };
    // Bit 16 of an LVT entry is the mask. Said in words as well as in hex so a
    // boot log is readable without the manual open beside it.
    if lvt & (1 << 16) != 0 {
        l.str(b" MASKED");
    }
    if svr & (1 << 8) == 0 {
        l.str(b" APIC-SOFTWARE-DISABLED");
    }
    l.end();
}
