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

// LAPIC timer (IRQ 0 / vector 0x20). EOI is issued by the IDT IRQ dispatcher
// after this returns (or, when a reschedule switches away, when the preempted
// task is resumed and unwinds back through the dispatcher).
//
// Runs the complete timer service: advance time, wake sleepers whose deadline
// passed, fire alarms/hooks, and act on the reschedule the tick raises. The
// previous minimal version only set NEED_RESCHEDULE with nothing to consume it
// and never woke timed sleepers, so a CPL=3 task that never yields (a
// busy-looping capsule) monopolized the core and timed waits never returned.
pub(super) fn timer_tick(_irq: u8) {
    // LAPIC end-of-interrupt FIRST. The generic IRQ dispatcher only issues a
    // PIC EOI (a no-op once the 8259 is masked), so without this the LAPIC
    // in-service bit for the timer vector stays set and the timer never fires
    // again after the first tick: no preemption, no sleeper wakeups, and a
    // busy CPL=3 capsule owns the core forever. Acking before the reschedule
    // also keeps the timer ticking while a switch runs another task.
    crate::arch::x86_64::interrupt::apic::send_eoi();
    crate::interrupts::on_timer_interrupt();
}
