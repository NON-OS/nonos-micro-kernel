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

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use spin::Mutex;

use super::types::{IpiFn, IpiWork, IpiWorkQueue};
use crate::arch::interrupt_controller::{broadcast_ipi, send_ipi, Ipi};
use crate::interrupts::disable_interrupts_guard;
use crate::smp::{cpu_count, cpus_online, get_cpu, MAX_CPUS};

pub use crate::smp::cpu_id;

// One lock per queue, not one lock over the array. The handler runs in
// interrupt context and takes the same lock the sender uses, so a CallFunction
// IPI landing on a cpu that was itself in the middle of a push would have
// spun in the handler on a lock only the interrupted code could release. Every
// critical section below therefore also masks interrupts, and splitting the
// lock keeps a cpu draining its own queue from blocking a sender aimed
// somewhere else.
static IPI_QUEUES: [Mutex<IpiWorkQueue>; MAX_CPUS] = {
    const INIT: Mutex<IpiWorkQueue> = Mutex::new(IpiWorkQueue::new());
    [INIT; MAX_CPUS]
};

static BARRIER_ARRIVED: AtomicU32 = AtomicU32::new(0);
static BARRIER_GENERATION: AtomicU32 = AtomicU32::new(0);
static BARRIER_TARGET: AtomicU32 = AtomicU32::new(0);

pub fn call_on_cpu(target_cpu: usize, func: IpiFn, arg: usize) -> Result<(), &'static str> {
    if target_cpu >= cpu_count() {
        return Err("Invalid CPU ID");
    }

    if target_cpu == cpu_id() {
        func(arg);
        return Ok(());
    }

    let cpu = get_cpu(target_cpu).ok_or("CPU not found")?;
    if !cpu.is_online() {
        return Err("CPU is offline");
    }

    let work = IpiWork { func, arg, done: AtomicBool::new(false) };

    {
        let _irq = disable_interrupts_guard();
        if !IPI_QUEUES[target_cpu].lock().push(work) {
            return Err("IPI queue full");
        }
    }

    send_ipi(cpu.get_apic_id(), Ipi::CallFunction).map_err(|_| "IPI not deliverable")?;

    Ok(())
}

pub fn call_on_all(func: IpiFn, arg: usize) {
    let count = cpu_count();
    let self_cpu = cpu_id();

    for cpu in 0..count {
        if cpu == self_cpu {
            func(arg);
        } else {
            let _ = call_on_cpu(cpu, func, arg);
        }
    }
}

pub fn call_on_others(func: IpiFn, arg: usize) {
    let count = cpu_count();
    let self_cpu = cpu_id();

    for cpu in 0..count {
        if cpu != self_cpu {
            let _ = call_on_cpu(cpu, func, arg);
        }
    }
}

/// Run everything queued for this cpu. Safe to call outside interrupt context:
/// a cpu spinning for a peer can drain its own queue by hand so the two do not
/// wait on each other.
pub fn handle_call_function_ipi() {
    let my_cpu = cpu_id();

    loop {
        // The work runs with the lock released. It can be arbitrarily long,
        // and a callback that queued more work would otherwise deadlock on a
        // lock its own caller still holds.
        let work = {
            let _irq = disable_interrupts_guard();
            IPI_QUEUES[my_cpu].lock().pop()
        };

        match work {
            Some(w) => {
                (w.func)(w.arg);
                w.done.store(true, Ordering::Release);
            }
            None => break,
        }
    }
}

pub fn barrier_all() {
    let target = cpus_online() as u32;
    let gen = BARRIER_GENERATION.fetch_add(1, Ordering::AcqRel);

    if gen > u32::MAX - 1000 {
        BARRIER_GENERATION.store(0, Ordering::Release);
    }

    BARRIER_TARGET.store(target, Ordering::Release);

    let _ = broadcast_ipi(Ipi::Barrier);

    let arrived = BARRIER_ARRIVED.fetch_add(1, Ordering::AcqRel) + 1;

    if arrived == target {
        BARRIER_ARRIVED.store(0, Ordering::Release);
    } else {
        while BARRIER_ARRIVED.load(Ordering::Acquire) != 0 {
            core::hint::spin_loop();
        }
    }
}

pub fn handle_barrier_ipi() {
    let target = BARRIER_TARGET.load(Ordering::Acquire);

    let arrived = BARRIER_ARRIVED.fetch_add(1, Ordering::AcqRel) + 1;

    if arrived == target {
        BARRIER_ARRIVED.store(0, Ordering::Release);
    } else {
        while BARRIER_ARRIVED.load(Ordering::Acquire) != 0 {
            core::hint::spin_loop();
        }
    }
}

pub fn broadcast_collect<T: Copy + Default>(func: fn(arg: usize) -> T, arg: usize) -> Vec<T> {
    let count = cpu_count();
    let mut results = alloc::vec![T::default(); count];

    for cpu in 0..count {
        if cpu == cpu_id() {
            results[cpu] = func(arg);
        }
    }

    results
}
