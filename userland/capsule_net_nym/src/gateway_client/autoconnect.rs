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

/// Register with the first bootstrap gateway that answers.
///
/// Tried in order and not at random: the list is short, and a client that
/// silently spreads itself across entry points gains nothing while making
/// failures harder to reproduce. Every candidate proves its identity key
/// during registration, so a dead or hostile entry costs a retry and cannot
/// impersonate a gateway.
///
/// Returns the index that succeeded.
pub fn autoconnect(tcp_port: u32) -> Option<usize> {
    for index in 0..BOOTSTRAP_GATEWAYS.len() {
        let candidate = bootstrap_gateway(index)?;
        if let Ok(gateway) = connect(tcp_port, candidate) {
            let _ = TABLE.lock().set_gateway(gateway);
            return Some(index);
        }
    }
    None
}
