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

// Capability bits (mirrors the kernel enum order used in format::CAP_TABLE).
// These are the authorities whose abuse would cross an isolation boundary, so
// the monitor counts who holds them. Holding one is not wrong on its own: a
// driver capsule needs raw hardware. The point is to make the attack surface
// visible rather than hidden.
pub const ADMIN: u64 = 1 << 9;
pub const DEBUG: u64 = 1 << 8;

pub const DRIVER: u64 = 1 << 16;
pub const MMIO: u64 = 1 << 17;
pub const IRQ: u64 = 1 << 18;
pub const DMA: u64 = 1 << 19;
pub const PIO: u64 = 1 << 20;

pub const SPAWN_BROKER: u64 = 1 << 23;
pub const SPAWN_WINDOW: u64 = 1 << 24;
pub const PROC_CTL: u64 = 1 << 25;

// Raw-hardware reach: any of these can touch physical devices directly. DMA is
// the sharpest edge because it bypasses the page tables entirely.
pub const RAW_HW: u64 = DRIVER | MMIO | IRQ | DMA | PIO;

// Authority over other processes: spawn them or end them.
pub const SPAWN: u64 = SPAWN_BROKER | SPAWN_WINDOW | PROC_CTL;

// Any sensitive authority at all. This is the union the Elevated filter selects
// on, so "elevated" means exactly the set the posture panel already counts.
pub const ANY: u64 = ADMIN | DEBUG | RAW_HW | SPAWN;
