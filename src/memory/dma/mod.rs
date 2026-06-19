// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

pub mod allocator;
pub mod coherency;
pub mod constants;
pub mod error;
pub mod pool;
mod stats;
pub mod types;

pub use allocator::*;
pub use coherency::{Coherency, DmaBuffer};
pub use constants::*;
pub use error::{DmaError, DmaResult};
pub use pool::DmaPool;
pub use types::{DmaConstraints, DmaDirection, DmaRegion, DmaStatsSnapshot, StreamingMapping};
