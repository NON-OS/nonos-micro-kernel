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

//! Asid-scoped TLB shootdown for every page-table mutation site in
//! the paging manager. Always issues the local `invlpg` first; on
//! multi-CPU runtime it then IPIs the peer CPUs running the same
//! asid (or every online CPU for a kernel-half flush). On single-CPU
//! runtime the broadcast block is skipped. Timeout policy is fail-
//! hard: a stale TLB entry would back freed DMA or MMIO, so an ack
//! that does not arrive inside `SHOOTDOWN_TIMEOUT_TSC` triggers a
//! panic-IPI broadcast and halts the originator.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use spin::Mutex;

use super::super::tlb;
use crate::arch::interrupt_controller::Ipi;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::smp::cpus_online;
use crate::smp::percpu::ASID_NONE;

/// `0` is the sentinel for "kernel half" or "no asid scoping". A
/// flush issued with `asid == ASID_KERNEL` reaches every online CPU
/// because the kernel half is shared across every address space.
pub const ASID_KERNEL: u32 = 0;

/// Bound on cross-CPU wait. ~10ms on a 1 GHz CPU, ~2.5ms on 4 GHz —
/// far longer than any healthy `invlpg` cycle. Tuned upwards is fine;
/// tuned to "wait forever" is forbidden.
const SHOOTDOWN_TIMEOUT_TSC: u64 = 10_000_000;

static SHOOTDOWN_LOCK: Mutex<()> = Mutex::new(());
static REQ_VA: AtomicU64 = AtomicU64::new(0);
static REQ_PAGES: AtomicU32 = AtomicU32::new(0);
static REQ_PENDING_ACKS: AtomicU32 = AtomicU32::new(0);

#[inline]
pub fn flush_tlb_one_smp(va: VirtAddr, asid: u32) {
    tlb::invalidate_page(va);
    if cpus_online() <= 1 {
        return;
    }
    broadcast(va, 1, asid);
}

#[inline]
pub fn flush_tlb_range_smp(start: VirtAddr, page_count: usize, asid: u32) {
    if page_count == 0 {
        return;
    }
    if page_count > 32 {
        flush_tlb_all_smp(asid);
        return;
    }
    for i in 0..page_count {
        let va = VirtAddr::new(start.as_u64() + (i * PAGE_SIZE_4K) as u64);
        tlb::invalidate_page(va);
    }
    if cpus_online() <= 1 {
        return;
    }
    broadcast(start, page_count as u32, asid);
}

#[inline]
pub fn flush_tlb_all_smp(asid: u32) {
    tlb::invalidate_all();
    if cpus_online() <= 1 {
        return;
    }
    // Encode "flush whole TLB" as page_count == 0 in the request
    // slot; the IPI handler treats that as `invalidate_all`.
    broadcast(VirtAddr::new(0), 0, asid);
}

fn broadcast(va: VirtAddr, page_count: u32, asid: u32) {
    // Serve any round already in flight while waiting for our turn. Page-table
    // mutation sites reach here with interrupts masked, so a cpu that simply
    // blocked on the lock could not answer the holder's IPI, and the two would
    // wait on each other until the timeout below halted the machine.
    let _guard = loop {
        if let Some(guard) = SHOOTDOWN_LOCK.try_lock() {
            break guard;
        }
        handle_shootdown_ipi();
        core::hint::spin_loop();
    };

    let self_cpu = crate::smp::cpu_id();
    let mut targets: u32 = 0;
    /*
     * Every cpu slot, filtered by whether it is running. Not `0..cpus_online()`:
     * that is a population count, while cpu numbers are handed out once per AP
     * attempted and are not reused when one fails. With a single failed AP the
     * live numbers are sparse, so counting up to the population both targets a
     * slot that never started, which can never acknowledge, and skips a cpu
     * that is running, which never gets the IPI. The wait below then always
     * reaches its deadline and halts the machine.
     */
    for cpu in 0..crate::smp::MAX_CPUS {
        if cpu == self_cpu || !crate::smp::cpu_is_online(cpu) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if !cpu_should_flush(d, asid) {
            continue;
        }
        // Mark the target before the request is published so it cannot be
        // read as "not my round" by a cpu that is already spinning here.
        d.tlb_flush_pending.store(1, Ordering::Relaxed);
        targets += 1;
    }
    if targets == 0 {
        return;
    }

    REQ_VA.store(va.as_u64(), Ordering::Release);
    REQ_PAGES.store(page_count, Ordering::Release);
    REQ_PENDING_ACKS.store(targets, Ordering::SeqCst);

    /*
     * The same slots as the marking loop above, for the same reason. The
     * pending flag is what selects the targets here, and only a cpu the loop
     * above could reach ever has it set, so the two must walk the same range.
     */
    for cpu in 0..crate::smp::MAX_CPUS {
        if cpu == self_cpu || !crate::smp::cpu_is_online(cpu) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if d.tlb_flush_pending.load(Ordering::Relaxed) == 0 {
            continue;
        }
        let _ = crate::arch::interrupt_controller::send_ipi(d.apic_id, Ipi::TlbShootdown);
    }
    wait_for_acks();
}

#[inline]
fn cpu_should_flush(data: &crate::smp::percpu::PerCpuData, asid: u32) -> bool {
    if asid == ASID_KERNEL {
        return true;
    }
    let active = data.active_asid.load(Ordering::Acquire);
    active != ASID_NONE && active == asid
}

/// Flush for the round in progress, if this cpu is one of its targets.
///
/// Driven by the TlbShootdown vector, and also called directly by a cpu
/// spinning for the lock in `broadcast`. The pending flag makes it safe either
/// way: it is what says the round applies to us, and clearing it before the
/// ack means neither path can acknowledge twice.
pub fn handle_shootdown_ipi() {
    let me = crate::smp::percpu::current();
    if me.tlb_flush_pending.swap(0, Ordering::AcqRel) == 0 {
        return;
    }
    let pages = REQ_PAGES.load(Ordering::Acquire);
    if pages == 0 {
        tlb::invalidate_all();
    } else {
        let base = VirtAddr::new(REQ_VA.load(Ordering::Acquire));
        for i in 0..pages as usize {
            let va = VirtAddr::new(base.as_u64() + (i * PAGE_SIZE_4K) as u64);
            tlb::invalidate_page(va);
        }
    }
    REQ_PENDING_ACKS.fetch_sub(1, Ordering::Release);
}

fn wait_for_acks() {
    let deadline = read_tsc().wrapping_add(SHOOTDOWN_TIMEOUT_TSC);
    while REQ_PENDING_ACKS.load(Ordering::Acquire) > 0 {
        if read_tsc() > deadline {
            crate::sys::serial::println(b"[FATAL] TLB shootdown timeout");
            report_stuck();
            crate::smp::send_panic_ipi();
            crate::arch::halt_loop();
        }
        core::hint::spin_loop();
    }
}

#[inline]
fn read_tsc() -> u64 {
    // SAFETY: eK@nonos.systems — rdtsc has no side effects and is
    // unconditionally available on every x86_64 CPU NØNOS supports.
    crate::arch::read_time_counter()
}

/// What every CPU looked like when the round gave up, printed before the halt.
///
/// A timeout says only that an acknowledgement did not arrive. Which CPU owed
/// it, whether that CPU was ever marked as a target, whether it is halted in
/// its idle loop or inside an interrupt handler, and whether it has taken a
/// timer interrupt since it came up are what separate "the IPI was never
/// delivered" from "the IPI was delivered and the CPU was in no position to
/// run it".
///
/// Each of those has to be read from something that is actually written. This
/// used to name interrupt-masking depth as well, and printed a field nothing
/// maintains.
fn report_stuck() {
    let mut head = crate::sys::serial::Line::new();
    head.str(b"[SMP] acks outstanding=").dec(REQ_PENDING_ACKS.load(Ordering::Acquire) as u64);
    head.end();
    for cpu in 0..crate::smp::MAX_CPUS {
        if !crate::smp::cpu_is_online(cpu) {
            continue;
        }
        let (Some(d), Some(desc)) = (crate::smp::percpu::get(cpu), crate::smp::get_cpu(cpu)) else {
            continue;
        };
        /*
         * One line per cpu, built whole. These are printed while the other
         * cpus are still running and printing, and a dump that interleaves
         * with them is unreadable exactly when it is needed.
         *
         * `irq_depth` comes from `interrupts::safety`, which the live handlers
         * maintain. This used to print `smp::percpu::irq_nesting` beside an
         * `interrupt_disable_depth`, and nothing writes either of them:
         * `enter_irq`, `leave_irq` and `in_irq` have no callers, and the
         * disable depth is only ever read here. Both columns were zero on
         * every cpu of every dump this kernel has ever produced, which reads
         * as a measurement and is a constant. The disable depth is gone rather
         * than reported, since there is nothing behind it to report.
         */
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[SMP]  cpu=").dec(cpu as u64);
        l.str(b" apic=").dec(d.apic_id as u64);
        l.str(b" pending=").dec(d.tlb_flush_pending.load(Ordering::Acquire) as u64);
        l.str(b" irq_depth=").dec(crate::interrupts::safety::depth_of(cpu) as u64);
        l.str(b" asid=").dec(d.active_asid.load(Ordering::Acquire) as u64);
        l.str(b" idle=").dec(u64::from(desc.idle.load(Ordering::Acquire)));
        l.str(b" idle_cycles=").dec(desc.idle_cycles.load(Ordering::Acquire));
        // Zero means this cpu has never taken a timer interrupt, which
        // separates "did not answer this round" from "has not answered
        // anything since it came up". Those need different fixes and the dump
        // could not tell them apart.
        l.str(b" ticked=").dec(u64::from(d.last_tick_tsc.load(Ordering::Acquire) != 0));
        // Where it was when it stopped answering. A halted CPU and one
        // spinning on a lock with interrupts masked are the same silence from
        // here, and they are not the same defect.
        l.str(b" at=").str(desc.stage().as_str().as_bytes());
        // What that CPU's own APIC had in service when it last looked. A vector
        // stuck here blocks its whole priority class and everything below it,
        // while leaving higher classes working, which is what a CPU taking
        // IPIs at 0x40 and no timer at 0x20 looks like from outside.
        match desc.in_service_seen.load(Ordering::Acquire) {
            0 => l.str(b" isr=unread"),
            1 => l.str(b" isr=none"),
            v => l.str(b" isr=").hex((v - 2) as u64),
        };
        l.end();
    }
}
