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

//! Moving onto a gateway the directory knows about.

use crate::state::TABLE;
use crate::topology;
use crate::trace;

/// Drop a gateway the directory does not describe.
///
/// The first gateway is dialled from the compiled list, because that is all
/// there is before a directory arrives. Once one has, that gateway is very
/// likely not in it, and a gateway the directory cannot describe is one no
/// route can end at: a reply comes home through the gateway holding the
/// session, and building that route needs the address and packet key the
/// directory publishes.
///
/// So the session is dropped rather than kept. The serve loop dials again on
/// the next idle moment, and by then the directory is what it picks from.
pub fn rebind_if_unknown() {
    let Some(gateway) = TABLE.lock().gateway() else {
        return;
    };
    // A directory with no gateways in it cannot offer a better one, and
    // dropping the session for it would put the capsule back on the compiled
    // list it just came from.
    if crate::state::directory_gateway_count() == 0 {
        return;
    }
    if topology::node_by_identity(&gateway.identity).is_some() {
        return;
    }
    trace::say(b"gateway is not in the directory, dialling one that is");
    super::gateway_lost();
}
