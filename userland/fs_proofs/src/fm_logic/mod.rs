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

// The self-contained file-manager logic, assembled from capsule source: the
// directory-listing parser and the content-type classifier. Modules that need
// the full app State are left out; their leaf helpers are covered here.

#[path = "../../../capsule_file_manager/src/fm/entries.rs"]
mod entries;
#[path = "../../../capsule_file_manager/src/fm/favorites.rs"]
pub mod favorites;
#[path = "../../../capsule_file_manager/src/fm/file_color.rs"]
mod file_color;
#[path = "../../../capsule_file_manager/src/fm/file_ext.rs"]
mod file_ext;
#[path = "../../../capsule_file_manager/src/fm/file_kind.rs"]
mod file_kind;
#[path = "../../../capsule_file_manager/src/fm/filetype.rs"]
mod filetype;
#[path = "../../../capsule_file_manager/src/fm/open_with_table.rs"]
pub mod open_with_table;
#[path = "../../../capsule_file_manager/src/fm/prefs.rs"]
pub mod prefs;
#[path = "../../../capsule_file_manager/src/fm/recents_filter.rs"]
pub mod recents_filter;
#[path = "../../../capsule_file_manager/src/fm/recents_group.rs"]
pub mod recents_group;
#[path = "../../../capsule_file_manager/src/fm/recents_tally.rs"]
pub mod recents_tally;

// Shim, not capsule source: see the header of `state.rs`.
pub mod state;

#[path = "../../../capsule_file_manager/src/fm/tags.rs"]
pub mod tags;

// Pure colour tokens, pulled in because `file_color` maps a file kind onto them.
#[path = "../../../capsule_file_manager/src/fm/theme.rs"]
#[allow(dead_code)]
mod theme;
#[path = "../../../capsule_file_manager/src/fm/tags_load.rs"]
mod tags_load;
#[path = "../../../capsule_file_manager/src/fm/tags_mutate.rs"]
mod tags_mutate;
#[path = "../../../capsule_file_manager/src/fm/tags_query.rs"]
mod tags_query;
#[path = "../../../capsule_file_manager/src/fm/tags_reconcile.rs"]
pub mod tags_reconcile;
#[path = "../../../capsule_file_manager/src/fm/undo.rs"]
pub mod undo;

pub use entries::{build_entries, Entry};
pub use file_color::color;
pub use file_ext::ext;
pub use file_kind::kind_of;
pub use filetype::Kind;
