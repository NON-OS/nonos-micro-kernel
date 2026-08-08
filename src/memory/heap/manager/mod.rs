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

mod api;
mod bootstrap;
mod globals;
mod init;
mod verify;

pub use api::get_allocator;
pub use api::{get_heap_stats, set_heap_zero_on_alloc, set_heap_zero_on_free};
pub use bootstrap::{init_bootstrap, is_using_bootstrap};
pub use globals::{get_timestamp, HEAP_STATS, HEAP_ZERO_ON_ALLOC, HEAP_ZERO_ON_FREE};
pub use init::init;
pub use verify::verify_heap_integrity;
