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
//! Memory-mapped I/O management.

extern crate alloc;

pub mod constants;
pub mod error;
pub mod manager;
mod ops;
pub mod ordering;
mod stats;
mod types;

pub use constants::*;
pub use error::{MmioError, MmioResult};
pub use manager::*;
pub use ops::{mmio_r16, mmio_r32, mmio_r64, mmio_r8, mmio_w16, mmio_w32, mmio_w64, mmio_w8};
pub use ordering::{fence_full, fence_reads, fence_writes, Mmio};
pub use stats::{MmioStats, MMIO_STATS};
pub use types::{MmioFlags, MmioRegion, MmioStatsSnapshot};
