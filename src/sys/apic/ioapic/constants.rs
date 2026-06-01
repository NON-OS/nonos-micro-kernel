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

// Default MMIO base when no MADT IOAPIC entry overrides it.
pub(super) const IOAPIC_DEFAULT_BASE: u64 = 0xFEC0_0000;

// Indirect data window offset from the IOAPIC register select port.
pub(super) const IOAPIC_WIN: u32 = 0x10;

// Indirect register addresses inside the IOAPIC.
pub(super) const IOAPIC_ID: u32 = 0x00;
pub(super) const IOAPIC_VER: u32 = 0x01;
pub(super) const IOAPIC_REDTBL: u32 = 0x10;

// Bit 16 of a low redirection entry masks the IRQ source.
pub(super) const REDIR_MASK_BIT: u32 = 0x1_0000;

// MPS / ACPI ISO polarity + trigger flag positions in the low redir
// entry. These match the IOAPIC RTE encoding directly.
pub(super) const REDIR_ACTIVE_LOW: u32 = 1 << 13;
pub(super) const REDIR_LEVEL_TRIGGERED: u32 = 1 << 15;
