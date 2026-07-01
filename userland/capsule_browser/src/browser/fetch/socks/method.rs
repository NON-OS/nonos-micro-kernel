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

use crate::browser::fetch::socks::{recv_some, request};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net;

pub fn method(port: u32, f: &mut Fetch) {
    recv_some::recv_some(port, f);
    if f.socks.len() < 2 || matches!(f.phase, Phase::Error) {
        return;
    }
    if f.socks[0] != 0x05 || f.socks[1] != 0x00 {
        f.error = Some("socks auth rejected");
        f.phase = Phase::Error;
        return;
    }
    let Some(req) = request::request(&f.url) else {
        f.error = Some("socks target rejected");
        f.phase = Phase::Error;
        return;
    };
    if net::socket_send(port, f.handle, &req).is_err() {
        f.error = Some("socks connect failed");
        f.phase = Phase::Error;
        return;
    }
    f.socks.clear();
    f.idle = 0;
    f.phase = Phase::SocksConnect;
}
