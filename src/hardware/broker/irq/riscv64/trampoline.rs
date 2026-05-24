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

use core::sync::atomic::Ordering;

use crate::arch::riscv64::plic;

use super::pending;

// PLIC dispatch hands us the source id. Outer trap path already
// `claim`s and will `complete` after we return; we just count the
// hit and mask further deliveries until the capsule's `MkIrqAck`.
pub(super) fn handle(source: u32) {
    let Some(e) = pending::find_by_source(source) else { return };
    let prev = e.pending.fetch_add(1, Ordering::AcqRel);
    if prev == u64::MAX {
        e.overflow.fetch_add(1, Ordering::Relaxed);
    }
    if plic::disable_irq(source).is_err() {
        e.overflow.fetch_add(1, Ordering::Relaxed);
    }
}
