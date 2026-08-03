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

use super::ops::connect;
use crate::state::{bootstrap_gateway, BOOTSTRAP_GATEWAYS, TABLE};

/// Try the bootstrap gateway at `index`, wrapping around the list.
///
/// One per call so the caller keeps control between attempts. Wrapping means
/// a client that starts before the network is up keeps trying.
pub fn connect_candidate(tcp_port: u32, index: usize) -> bool {
    let slot = index % BOOTSTRAP_GATEWAYS.len();
    let Some(candidate) = bootstrap_gateway(slot) else {
        return false;
    };
    match connect(tcp_port, candidate) {
        Ok(gateway) => {
            let _ = TABLE.lock().set_gateway(gateway);
            super::trace::bound(gateway.ip);
            true
        }
        Err(_) => false,
    }
}
