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

//! Kernel synchronisation primitives that are SMP-safe and also safe
//! against same-CPU recursion from interrupt context. The standard
//! `spin::Mutex` serialises between CPUs but a syscall holding it can
//! be preempted, the trap handler can try to re-acquire it on the
//! same CPU, and the spin never breaks. The wrappers in this module
//! disable interrupts on acquire and restore the prior IF state on
//! drop, so neither a same-CPU ISR nor a peer CPU can wedge.
//!
//! Use the irq-safe wrappers for any lock that can be touched from
//! both a syscall and an interrupt handler — broker tables (claim,
//! grant, irq records, dma records, pio grants), the service
//! registry, and the ipc inbox registry.

pub mod irq_mutex;
pub mod irq_rwlock;

pub use irq_mutex::{IrqMutex, IrqMutexGuard};
pub use irq_rwlock::{IrqRwLock, IrqRwLockReadGuard, IrqRwLockWriteGuard};
