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

use crate::capabilities::caps_to_bits;
use crate::services::registry::{lookup_port, lookup_service};
use crate::syscall::caps::current_caps_or_default;

/// Whether this caller may send to `endpoint`.
///
/// An endpoint with no stated requirement is refused, not admitted. Every
/// registration path sets one: service endpoints take at least `IPC` from
/// `required_caps`, and a reply inbox is claimed with `IPC` the moment its
/// owner exists. A requirement of zero therefore means an endpoint that was
/// never finished being set up, and the safe reading of that is no.
///
/// This used to return true, which made an unfinished endpoint reachable by
/// anyone and turned every capability on it into a suggestion.
///
/// An unknown name is refused for the same reason. Falling back to a permitted
/// send would let a caller reach an endpoint simply by naming one that does
/// not exist yet, and win the race when it appears.
pub(super) fn caller_satisfies_endpoint(endpoint: u64, target: &str) -> bool {
    let Some(required) = lookup_service(target)
        .or_else(|| lookup_port(endpoint as u32))
        .map(|ep| ep.caps_required)
    else {
        return false;
    };
    if required == 0 {
        return false;
    }
    caps_to_bits(&current_caps_or_default().permissions) & required == required
}
