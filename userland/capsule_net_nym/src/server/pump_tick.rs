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

use crate::server::handlers::drain_stream;

/// Read the gateway on an idle tick.
///
/// A reply arrives when the mixnet is done with it, not when a client asks,
/// and the delay is seconds of deliberate mixing. Draining only inside a
/// client's read meant the answer had to arrive during the one moment someone
/// was looking, and every reply that missed that window sat in the socket
/// until a later read happened to find it.
///
/// Nothing is waited for here. Whatever the link already holds is taken and
/// queued against the session it belongs to, so the client's next read finds
/// it waiting rather than having to catch it in flight.
pub fn pump_tick() {
    drain_stream(0);
}
