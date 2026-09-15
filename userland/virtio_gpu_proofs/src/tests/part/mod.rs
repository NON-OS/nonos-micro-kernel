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

//! A part on its own thread, consuming the control queue.
//!
//! It walks the available ring the way a device does: each new head names a
//! descriptor chain, the first descriptor is the request, the second is the
//! buffer the answer goes in, and completion is a used-ring entry naming the
//! head. `Answer` picks how it replies; `Spec` is a conforming part.

pub mod answers;
mod bodies;
mod ring;
mod serve;

mod answering;
mod types;

pub use answering::answering;
pub use types::{Answer, Part, Seen};
