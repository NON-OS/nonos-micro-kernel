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

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use super::operations::cpu_id;

pub const MAX_IPI_WORK: usize = 16;

pub type IpiFn = fn(arg: usize);

pub struct IpiWork {
    pub func: IpiFn,
    pub arg: usize,
    pub done: AtomicBool,
}

pub struct IpiWorkQueue {
    pub items: [Option<IpiWork>; MAX_IPI_WORK],
    pub head: usize,
    pub tail: usize,
}

impl IpiWorkQueue {
    pub const fn new() -> Self {
        const NONE: Option<IpiWork> = None;
        Self { items: [NONE; MAX_IPI_WORK], head: 0, tail: 0 }
    }

    pub fn push(&mut self, work: IpiWork) -> bool {
        let next_tail = (self.tail + 1) % MAX_IPI_WORK;
        if next_tail == self.head {
            return false;
        }
        self.items[self.tail] = Some(work);
        self.tail = next_tail;
        true
    }

    pub fn pop(&mut self) -> Option<IpiWork> {
        if self.head == self.tail {
            return None;
        }
        let item = self.items[self.head].take();
        self.head = (self.head + 1) % MAX_IPI_WORK;
        item
    }
}

pub struct OnceBarrier {
    done: AtomicBool,
    running: AtomicBool,
}

impl OnceBarrier {
    pub const fn new() -> Self {
        Self { done: AtomicBool::new(false), running: AtomicBool::new(false) }
    }

    pub fn call_once<F: FnOnce()>(&self, f: F) -> bool {
        if self.done.load(Ordering::Acquire) {
            return false;
        }

        if self.running.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            f();
            self.done.store(true, Ordering::Release);
            return true;
        }

        while !self.done.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }
        false
    }
}

pub struct PerCpuLock {
    owner: AtomicU32,
    depth: AtomicU32,
}

impl PerCpuLock {
    pub const fn new() -> Self {
        Self { owner: AtomicU32::new(u32::MAX), depth: AtomicU32::new(0) }
    }

    pub fn lock(&self) {
        let cpu = cpu_id() as u32;

        if self.owner.load(Ordering::Acquire) == cpu {
            self.depth.fetch_add(1, Ordering::Relaxed);
            return;
        }

        while self
            .owner
            .compare_exchange(u32::MAX, cpu, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            while self.owner.load(Ordering::Relaxed) != u32::MAX {
                core::hint::spin_loop();
            }
        }

        self.depth.store(1, Ordering::Relaxed);
    }

    pub fn unlock(&self) {
        let depth = self.depth.fetch_sub(1, Ordering::Relaxed);
        if depth == 1 {
            self.owner.store(u32::MAX, Ordering::Release);
        }
    }

    pub fn try_lock(&self) -> bool {
        let cpu = cpu_id() as u32;

        if self.owner.load(Ordering::Acquire) == cpu {
            self.depth.fetch_add(1, Ordering::Relaxed);
            return true;
        }

        if self.owner.compare_exchange(u32::MAX, cpu, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.depth.store(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
}
