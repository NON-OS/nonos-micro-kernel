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

//! Refs: `HEAD` and the branch files under `refs/heads`.

mod head;
mod name;
mod read_head;
mod resolve_head;
mod set_head_branch;
mod update_ref;

pub use head::Head;
pub use name::is_valid_ref_name;
pub use read_head::read_head;
pub use resolve_head::resolve_head;
pub use set_head_branch::set_head_branch;
pub use update_ref::update_ref;
