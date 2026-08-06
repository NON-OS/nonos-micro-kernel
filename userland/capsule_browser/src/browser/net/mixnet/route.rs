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

use alloc::vec::Vec;
use spin::Mutex;

/// Where the browser's bytes go.
///
/// `net.socks5` speaks RFC 1928 over IPC and has no listening socket, so it
/// cannot be bypassed by dialling past it. This holds the state that makes a
/// service call look like a socket to the code above.
pub struct Route {
    pub socks_port: u32,
    /// Bytes the proxy has answered that the reader has not taken yet.
    pub pending: Vec<u8>,
    /// The proxy has said the far end finished, so no further asking will
    /// produce anything.
    pub closed: bool,
}

static ROUTE: Mutex<Option<Route>> = Mutex::new(None);

/// Send everything through `net.socks5` from now on.
pub fn enable(socks_port: u32) {
    *ROUTE.lock() = Some(Route { socks_port, pending: Vec::new(), closed: false });
}

/// Go back to reaching hosts directly.
pub fn disable() {
    *ROUTE.lock() = None;
}

pub fn is_on() -> bool {
    ROUTE.lock().is_some()
}

/// Run `f` against the route, or report that there is none.
pub fn with<R>(f: impl FnOnce(&mut Route) -> R) -> Result<R, ()> {
    let mut guard = ROUTE.lock();
    match guard.as_mut() {
        Some(route) => Ok(f(route)),
        None => Err(()),
    }
}
