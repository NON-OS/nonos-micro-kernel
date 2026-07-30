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

use super::state::{cpus_online, TLB_SHOOTDOWN_ACK, TLB_SHOOTDOWN_ACTIVE, TLB_SHOOTDOWN_ADDR};
use crate::arch::interrupt_controller::{broadcast_ipi, Ipi};
use crate::arch::paging::{invalidate_all, invalidate_page};
use crate::memory::addr::VirtAddr;
use core::sync::atomic::Ordering;

/// How long to wait for the other CPUs to acknowledge, in cycle-counter ticks.
const ACK_TIMEOUT_TICKS: u64 = 10_000_000;

pub fn tlb_shootdown(addr: VirtAddr) {
    if cpus_online() <= 1 {
        invalidate_page(addr.as_u64());
        return;
    }

    TLB_SHOOTDOWN_ADDR.store(addr.as_u64(), Ordering::Release);
    TLB_SHOOTDOWN_ACK.store(0, Ordering::Release);
    TLB_SHOOTDOWN_ACTIVE.store(true, Ordering::Release);

    if broadcast_ipi(Ipi::TlbShootdown).is_err() {
        // No other CPU was told to flush, so waiting for acknowledgements
        // would only burn the timeout. The local invalidation below still has
        // to happen either way.
        crate::log_error!("[SMP] TLB shootdown broadcast refused");
        TLB_SHOOTDOWN_ACTIVE.store(false, Ordering::Release);
        invalidate_page(addr.as_u64());
        return;
    }

    invalidate_page(addr.as_u64());

    let expected = cpus_online() as u32 - 1;
    let start = read_ticks();

    while TLB_SHOOTDOWN_ACK.load(Ordering::Acquire) < expected {
        if read_ticks().wrapping_sub(start) > ACK_TIMEOUT_TICKS {
            crate::log_error!("[SMP] TLB shootdown timeout");
            break;
        }
        core::hint::spin_loop();
    }

    TLB_SHOOTDOWN_ACTIVE.store(false, Ordering::Release);
}

pub fn handle_tlb_shootdown_ipi() {
    if TLB_SHOOTDOWN_ACTIVE.load(Ordering::Acquire) {
        invalidate_page(TLB_SHOOTDOWN_ADDR.load(Ordering::Acquire));
        TLB_SHOOTDOWN_ACK.fetch_add(1, Ordering::Release);
    }
}

/// Drop every entry in the calling CPU's TLB.
pub fn flush_tlb() {
    invalidate_all();
}

#[inline]
fn read_ticks() -> u64 {
    crate::arch::read_time_counter()
}
