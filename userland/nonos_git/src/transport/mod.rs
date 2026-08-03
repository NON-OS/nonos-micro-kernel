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
//! How bytes reach a remote.
//!
//! The protocol code here does not open sockets or speak TLS. It asks for a
//! request and is handed a response, so the same fetch runs against a real
//! server in the shell and against recorded bytes in a test.

mod error;
mod traits;

pub use error::TransportError;
pub use traits::Transport;
