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

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8};

use super::constants::IOAPIC_DEFAULT_BASE;

// MMIO base; overridden by MADT on machines that report a different one.
pub(super) static IOAPIC_BASE: AtomicU64 = AtomicU64::new(IOAPIC_DEFAULT_BASE);

// First GSI handled by this IOAPIC. Only the primary IOAPIC is driven.
pub(super) static IOAPIC_GSI_BASE: AtomicU32 = AtomicU32::new(0);

// One past the largest GSI handled by this IOAPIC. Populated from the
// IOAPICVER register on init. 24 is the legacy IBM PC count.
pub(super) static IOAPIC_MAX_REDIR: AtomicU8 = AtomicU8::new(24);

// Init guard; reads are cheap so the rest of the kernel can call into
// the surface without worrying about ordering.
pub static IOAPIC_INIT: AtomicBool = AtomicBool::new(false);
