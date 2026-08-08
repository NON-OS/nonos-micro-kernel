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

use crate::browser::net::{lookup, mixnet};

const PROXY: &[u8] = b"net.socks5";

/// Route through the mixnet whenever the proxy capsule is running.
///
/// Presence decides it, not a setting: going direct while a mixnet proxy sits
/// there would publish the address it exists to hide, and the page would look
/// the same either way. A proxy that cannot carry a request fails it rather
/// than falling back, since reverting quietly is the disclosure.
pub fn route_mixnet() {
    // The reader can say they do not want this session hidden. Honouring it
    // here, before a route is taken, is what makes it a choice rather than a
    // fallback: nothing reverts on failure, and a request the mixnet cannot
    // carry still fails instead of quietly going direct.
    if !mixnet::wanted() {
        if mixnet::is_on() {
            mixnet::disable();
        }
        return;
    }
    if mixnet::is_on() {
        return;
    }
    let port = lookup(PROXY);
    if port != 0 {
        mixnet::enable(port);
    }
}
