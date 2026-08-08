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

use super::carve::carve;

static MMIO_NEXT: AtomicU64 = AtomicU64::new(0);
static MMIO_END: AtomicU64 = AtomicU64::new(0);
static IO_NEXT: AtomicU64 = AtomicU64::new(0);
static IO_END: AtomicU64 = AtomicU64::new(0);

/// Record the address ranges the host bridge forwards to its devices. Both are
/// bus addresses: what gets written into a BAR, not where the CPU reaches it.
pub fn set_windows(mmio_base: u64, mmio_size: u64, io_base: u64, io_size: u64) {
    MMIO_NEXT.store(mmio_base, Ordering::SeqCst);
    MMIO_END.store(mmio_base.saturating_add(mmio_size), Ordering::SeqCst);
    IO_NEXT.store(io_base, Ordering::SeqCst);
    IO_END.store(io_base.saturating_add(io_size), Ordering::SeqCst);
}

/// Carve `size` bytes out of the memory window. `None` once it is exhausted,
/// which leaves the BAR unassigned rather than handing back an address the
/// bridge does not forward.
pub(super) fn alloc_mmio(size: u64) -> Option<u64> {
    alloc_from(&MMIO_NEXT, &MMIO_END, size)
}

/// Same, for the bridge's I/O window.
pub(super) fn alloc_io(size: u64) -> Option<u64> {
    alloc_from(&IO_NEXT, &IO_END, size)
}

/// Advance one window's cursor, retrying when another CPU wins the exchange.
fn alloc_from(next: &AtomicU64, end: &AtomicU64, size: u64) -> Option<u64> {
    let limit = end.load(Ordering::SeqCst);
    loop {
        let cursor = next.load(Ordering::SeqCst);
        let (base, after) = carve(cursor, limit, size)?;
        if next.compare_exchange(cursor, after, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            return Some(base);
        }
    }
}
