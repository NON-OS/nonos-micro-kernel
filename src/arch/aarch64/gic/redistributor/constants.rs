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

pub(super) const GICR_TYPER: u64 = 0x0008;
pub(super) const GICR_WAKER: u64 = 0x0014;
pub(super) const GICR_SGI_BASE: u64 = 0x10000;
pub(super) const GICR_IGROUPR0: u64 = GICR_SGI_BASE + 0x0080;
pub(super) const GICR_ISENABLER0: u64 = GICR_SGI_BASE + 0x0100;
pub(super) const GICR_ICENABLER0: u64 = GICR_SGI_BASE + 0x0180;
pub(super) const GICR_ISPENDR0: u64 = GICR_SGI_BASE + 0x0200;
pub(super) const GICR_ICPENDR0: u64 = GICR_SGI_BASE + 0x0280;
pub(super) const GICR_IPRIORITYR: u64 = GICR_SGI_BASE + 0x0400;
pub(super) const GICR_ICFGR0: u64 = GICR_SGI_BASE + 0x0C00;
pub(super) const GICR_ICFGR1: u64 = GICR_SGI_BASE + 0x0C04;
pub(super) const GICR_IGRPMODR0: u64 = GICR_SGI_BASE + 0x0D00;
pub(super) const WAKER_PROCESSOR_SLEEP: u32 = 1 << 1;
pub(super) const WAKER_CHILDREN_ASLEEP: u32 = 1 << 2;
