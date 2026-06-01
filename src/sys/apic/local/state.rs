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

use core::sync::atomic::{AtomicBool, AtomicU64};

use super::constants::LOCAL_APIC_DEFAULT_BASE;

// Current LAPIC base. Starts at the boot identity-mapped physical
// address and gets republished to the kernel-half UC virtual mapping
// by VM init via rebind_to_virt before the low identity map is torn
// down. The store is atomic so an interleaved EOI() reads either the
// old or new base, never a torn or unmapped pointer.
pub(super) static LAPIC_BASE: AtomicU64 = AtomicU64::new(LOCAL_APIC_DEFAULT_BASE);

// Init guard for the surface; readers in eoi() and timer fast path
// skip work when this is false rather than risk poking an uninit base.
pub static LAPIC_INIT: AtomicBool = AtomicBool::new(false);
