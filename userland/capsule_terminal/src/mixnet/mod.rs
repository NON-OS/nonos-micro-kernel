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

//! Reaching hosts through the mixnet from the command line.
//!
//! The browser routes through `net.socks5` whenever that capsule is running.
//! Anything the terminal fetches, `curl` and `git` included, leaves the same
//! way for the same reason, so a shell is not the hole in a machine that is
//! otherwise anonymised.

mod socks;
mod stream;
mod wire;

pub use stream::Wire;

/// Whether the mixnet proxy is running, and so whether anything that leaves
/// directly is worth pointing out.
pub fn routed() -> bool {
    stream::proxy_available()
}
