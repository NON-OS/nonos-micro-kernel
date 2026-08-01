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

//! VT-d domain and device bookkeeping. No device is confined by this module:
//! DMA remapping is never enabled. Domains and bindings are tracked in
//! software, but no root table, context entry or second-level page table is
//! written and Translation Enable is never set, so every DMA-capable device
//! can reach all of physical memory. Calls that would imply isolation return
//! `NotEnforcing` rather than success. The tables are kept because a real
//! implementation programs from them.

pub mod device;
pub mod domain;
pub mod globals;
pub mod mapping;
pub mod regs;
pub mod tables;
pub mod types;
pub mod unit;
