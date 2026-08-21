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

mod agaw;
mod behaviour;
mod fault;
mod limits;
mod pages;

pub use agaw::{preferred_levels, AgawLevels};
pub use behaviour::{caching_mode, requires_write_buffer_flush};
pub use fault::{fault_recording_count, fault_recording_offset};
pub use limits::{domain_count, max_address_width};
pub use pages::best_leaf_level;
