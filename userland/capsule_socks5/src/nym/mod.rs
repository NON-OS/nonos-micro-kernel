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

//! Carrying SOCKS streams over the mixnet.

mod exit;
mod send;
mod session;

pub use exit::{exit, set_exit, Exit};
pub use send::{connect_request, SendError};
pub use session::{open_session, session};

/// Serve SOCKS clients. Not yet wired to an IPC listener.
pub fn serve() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
