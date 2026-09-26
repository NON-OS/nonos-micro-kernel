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

//! The capability table, from the kernel that enforces it.
//!
//! The mask in a receipt means whatever the kernel's `bit()` says it means, so
//! reading it against any other table would be reading a different machine's
//! answer. These four files are the kernel's, unmodified: a capability added
//! there appears here, and one renumbered there breaks this build.

#[path = "../../../../../src/capabilities/types/defs.rs"]
pub mod defs;

#[path = "../../../../../src/capabilities/types/table.rs"]
mod table;

#[path = "../../../../../src/capabilities/types/as_str.rs"]
mod as_str;

#[path = "../../../../../src/capabilities/types/guard.rs"]
mod guard;

pub use defs::Capability;
