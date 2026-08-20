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

//! The most authority any capsule in this image may hold.
//!
//! Per-capsule bits answer "what can this program do". Answering "can anything
//! here reach the network" from those means enumerating every capsule and
//! trusting the enumeration, which a remote party cannot do. This is one
//! value, enforced on the path every capsule takes, and folded into the
//! attestation.

mod admits;
mod value;

pub use admits::{admits, excess, report, report_pid};
pub use value::{ceiling, is_restricted};
