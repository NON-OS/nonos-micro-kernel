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

use crate::orchestrator::enumerate::HidEndpoint;
use crate::state::State;

use super::constants::HID_REPORT_MAX;
use super::drain_endpoint::drain_endpoint;

pub(super) fn drain_endpoints(
    state: &mut State,
    eps: &[HidEndpoint],
    buf: &mut [u8; HID_REPORT_MAX],
) -> bool {
    let mut drained = false;
    for ep in eps {
        drained |= drain_endpoint(state, ep, buf);
    }
    drained
}
