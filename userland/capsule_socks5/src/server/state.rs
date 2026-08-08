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

use super::clients::Clients;
use super::inbox::Inbox;
use crate::manager::Manager;
use spin::Mutex;

/// The handshakes in flight, and the tunnel table behind them.
///
/// Handshakes are per caller because one page load opens several at once. The
/// tunnels they produce all ride the single mixnet session, which is what
/// keeps every capsule's traffic in one stream rather than in a stream of its
/// own. Slots are keyed on the pid the kernel attests at delivery.
pub struct Server {
    pub clients: Clients,
    pub manager: Manager,
    pub inbox: Inbox,
}

pub static SERVER: Mutex<Option<Server>> = Mutex::new(None);

/// Reset the server, discarding every handshake in flight.
pub fn reset() {
    *SERVER.lock() =
        Some(Server { clients: Clients::new(), manager: Manager::new(), inbox: Inbox::default() });
}
