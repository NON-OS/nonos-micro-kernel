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

//! Desktop filesystem state: loading the root into icons and creating new
//! entries from the right-click menu.

mod commit_rename;
mod create_entry;
mod delete_entry;
mod drag_grab;
mod move_into;
mod refresh;
mod release_keys;
mod rename_key;
mod same;
mod start_rename;
mod unique_name;

pub use create_entry::create_entry;
pub use delete_entry::delete_entry;
pub use drag_grab::{grab_drag, release_drag};
pub use move_into::move_into;
pub use refresh::refresh;
pub use rename_key::rename_key;
pub use start_rename::start_rename;
