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

//! Host callbacks the QuickJS bindings call that the first bridge did not
//! cover. They are reached from C by name, so nothing here is referenced
//! from Rust and the module exists to keep the surface in one place.

mod clone;
mod edit;
mod location;
mod nav;
mod query;

/// How far up a parent chain a walk will go before giving up.
///
/// A tree a script built can hold a cycle, and a walk up it would otherwise
/// never end.
const MAX_ANCESTRY: u32 = 512;
