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
/// Presence decides this rather than a setting. A browser that reaches hosts
/// directly while a mixnet proxy is available would publish the address the
/// proxy exists to hide, and nothing on the page would show that it had. The
/// safe default is therefore the private one, and a direct path is what has
/// to be asked for.
///
/// Once routed, a proxy that cannot carry the request fails it. There is no
/// fall back to the direct path: silently reverting would turn one failed
/// request into a disclosure.
pub fn route_mixnet() {
    if mixnet::is_on() {
        return;
    }
    let port = lookup(PROXY);
    if port != 0 {
        mixnet::enable(port);
    }
}
