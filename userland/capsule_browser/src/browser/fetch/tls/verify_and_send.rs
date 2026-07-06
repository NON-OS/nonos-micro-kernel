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

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::http;
use crate::browser::net;
use crate::browser::tls13;

pub(in crate::browser::fetch) fn verify_and_send(port: u32, f: &mut Fetch) {
    let req = http::request::build(&f.url, f.post.as_deref());
    let host = f.url.host.clone();
    let Some(tls) = f.tls.as_ref() else {
        f.phase = Phase::Error;
        return;
    };
    let Some(out) =
        tls13::application_write(&tls.cf, &tls.flight, req.as_bytes(), host.as_bytes(), tls.now)
    else {
        f.error = Some("cert verify failed");
        f.phase = Phase::Error;
        return;
    };
    // The handshake is settled and its inputs no longer change, so derive the
    // server application keys once now and cache them; response records then
    // decrypt without repeating certificate verification on every read tick.
    let server_app =
        tls13::server_complete(&tls.cf, &tls.flight, host.as_bytes(), tls.now).map(|sc| sc.app);
    if net::socket_send(port, f.handle, &out).is_err() {
        f.error = Some("send failed");
        f.phase = Phase::Error;
        return;
    }
    if let Some(app) = server_app {
        if let Some(tls) = f.tls.as_mut() {
            tls.server_app = Some(app);
        }
    }
    f.buf.clear();
    f.phase = Phase::ReadBody;
}
