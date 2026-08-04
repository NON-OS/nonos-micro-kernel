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

//! Reading a live node list out of a Nym API response.
//!
//! The compiled-in tables are a snapshot: they were true when the image was
//! built and get no less wrong with time. This turns an API answer into the
//! same node records, so the network view is what the network currently is.

mod base58;
mod field;
mod node;
mod objects;
#[cfg(test)]
mod vectors;

pub use node::parse_node;
pub use objects::objects;
