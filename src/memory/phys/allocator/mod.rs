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

mod alloc;
mod api;
mod contiguous;
mod init;
mod query;
mod random;
mod zeroing;

pub use alloc::{allocate_frame, deallocate_frame};
pub use api::*;
pub use contiguous::{allocate_contiguous, free_contiguous};
pub use init::init_with_bitmap;
pub use query::{get_zone_stats, largest_free_run, managed_range, total_memory};
pub use random::{derive_seed, mix64};
pub use zeroing::zero_frame;
