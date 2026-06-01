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

use super::state::LAPIC_BASE;

// Atomically republish the LAPIC register base. Called once during
// VM init with a permanent UC kernel-half virtual mapping of the
// physical LAPIC page, before the bootloader low identity map (which
// the raw-physical base depended on) is torn down. The store is
// atomic, so an interleaved timer-IRQ `eoi()` reads either the old
// (still identity-mapped) or new (UC-mapped) base — never a torn or
// unmapped address.
pub fn rebind_to_virt(va: u64) {
    LAPIC_BASE.store(va, Ordering::SeqCst);
}
