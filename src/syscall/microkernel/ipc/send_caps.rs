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

// A service may register an endpoint with required capability bits. The send
// path must refuse a caller that does not hold them; an endpoint with no
// requirement (caps_required == 0) is open to any IPC sender.
pub(super) fn caller_satisfies_endpoint(endpoint: u64, target: &str) -> bool {
    let required = lookup_service(target)
        .or_else(|| lookup_port(endpoint as u32))
        .map(|ep| ep.caps_required)
        .unwrap_or(0);
    if required == 0 {
        return true;
    }
    caps_to_bits(&current_caps_or_default().permissions) & required == required
}
