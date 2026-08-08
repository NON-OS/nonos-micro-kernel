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

//! DMA sync windows where the sync is not a formality.
//!
//! An x86 host bus snoops the caches, so both windows collapse to a fence.
//! Plenty of arm64 systems do not: a device reading a buffer sees main memory
//! while the CPU's writes sit in a dirty line, and a device writing one leaves
//! the CPU reading what it cached earlier. Skipping the maintenance does not
//! fail loudly, it corrupts quietly.

mod maintain;
mod sync;

pub(in crate::memory::dma::coherency) use sync::{sync_for_cpu, sync_for_device};
