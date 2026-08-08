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

//! Sphinx wire constants.
//!
//! Written as the expressions the reference derives them from rather than the
//! numbers they evaluate to, so they can be checked against the specification
//! without arithmetic and a change upstream shows up as a changed formula
//! instead of a stale literal.

mod fields;
mod flags;
mod kdf;
mod sizes;
mod version;

pub use fields::*;
pub use flags::*;
pub use kdf::*;
pub use sizes::*;
pub use version::PACKET_VERSION;
