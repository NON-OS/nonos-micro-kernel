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

pub(super) const GICD_CTLR: u64 = 0x0000;
pub(super) const GICD_TYPER: u64 = 0x0004;
pub(super) const GICD_IGROUPR: u64 = 0x0080;
pub(super) const GICD_ISENABLER: u64 = 0x0100;
pub(super) const GICD_ICENABLER: u64 = 0x0180;
pub(super) const GICD_ISPENDR: u64 = 0x0200;
pub(super) const GICD_ICPENDR: u64 = 0x0280;
pub(super) const GICD_IPRIORITYR: u64 = 0x0400;
pub(super) const GICD_ITARGETSR: u64 = 0x0800;
pub(super) const GICD_ICFGR: u64 = 0x0C00;
pub(super) const GICD_IROUTER: u64 = 0x6000;
pub(super) const CTLR_ENABLE_G0: u32 = 1 << 0;
pub(super) const CTLR_ENABLE_G1NS: u32 = 1 << 1;
pub(super) const CTLR_ARE_S: u32 = 1 << 4;
pub(super) const CTLR_ARE_NS: u32 = 1 << 5;
