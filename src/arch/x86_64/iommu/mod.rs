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

//! VT-d domain and device bookkeeping.
//!
//! Read this before trusting anything here: **no device on this machine is
//! confined by this module.** DMA remapping is not enabled. What exists is the
//! software side, and it stops short of the hardware.
//!
//! What runs today: the ACPI DMAR parse finds the remapping units and records
//! their register bases, domains can be allocated and freed, and devices can
//! be recorded against a domain. What does not exist: any write to a remapping
//! unit. No root table is installed, no context entry is programmed, no
//! second-level page table is built, and the Translation Enable bit is never
//! set. The DRHD register bases the parse collected have no reader.
//!
//! So every call that would otherwise mean "this device is now isolated"
//! returns `VtdError::NotEnforcing` rather than success. `map_device` used to
//! return `Ok(())` after recording a binding, which told a caller its device
//! was confined while that device still had all of physical memory. Nothing
//! calls these yet, and the refusal is there so the first caller cannot
//! inherit that belief.
//!
//! The consequence for real hardware is worth stating plainly: every
//! DMA-capable device, network, storage, USB, and the GPU above all, can read
//! and write any physical address. The bookkeeping is kept because a real
//! implementation programs from exactly these tables.

pub mod device;
pub mod domain;
pub mod globals;
pub mod mapping;
pub mod types;
