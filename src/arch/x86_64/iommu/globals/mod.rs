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

mod allocate_domain_id;
mod is_enforcing;
mod is_present;
mod page_levels;
mod set_present;
pub(super) mod state;

pub use allocate_domain_id::allocate_domain_id;
pub use is_enforcing::{is_enforcing, set_enforcing};
pub use is_present::is_present;
pub use page_levels::{page_levels, set_page_levels};
pub use set_present::set_present;
