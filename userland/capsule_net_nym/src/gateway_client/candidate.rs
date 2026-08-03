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
/// One candidate per call so the caller keeps control: a connection now waits
/// a real round trip at each stage, and whoever is driving this has other work
/// to do between attempts. Wrapping rather than stopping means a client that
/// starts before the network is up keeps trying rather than giving up on the
/// whole list once.
pub fn connect_candidate(tcp_port: u32, index: usize) -> bool {
    let slot = index % BOOTSTRAP_GATEWAYS.len();
    let Some(candidate) = bootstrap_gateway(slot) else {
        return false;
    };
    match connect(tcp_port, candidate) {
        Ok(gateway) => {
            let _ = TABLE.lock().set_gateway(gateway);
            // Only failures were reported, so a session that established left
            // no trace at all and had to be inferred from an absence of
            // errors. Say when the mixnet is reachable.
            super::trace::bound(gateway.ip);
            true
        }
        Err(_) => false,
    }
}
