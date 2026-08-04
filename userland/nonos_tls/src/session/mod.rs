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
//! A blocking TLS session.
//!
//! The browser drives the handshake as a state machine because it cannot
//! block its UI. A caller that can wait wants none of that, so this walks the
//! same steps in order and hands back the response.

mod exchange;
mod flight;
mod response;
mod settled;
mod traits;

pub use exchange::exchange;
pub use traits::{Io, SessionError};
