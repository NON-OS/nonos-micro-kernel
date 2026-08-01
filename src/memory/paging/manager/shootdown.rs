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
    let count = cpus_online();
    let mut targets: u32 = 0;
    for cpu in 0..count {
        if cpu == self_cpu {
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

    for cpu in 0..count {
        if cpu == self_cpu {
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
