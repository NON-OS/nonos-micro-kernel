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
//! Laying the request out as bytes.

extern crate alloc;

use alloc::vec::Vec;

use super::build::{Request, RequestBuilder};
use super::write::{decimal, push};

impl RequestBuilder<'_> {
    /// Serialise. The connection is closed after the response, which costs a
    /// handshake per request but means no state survives to be got wrong.
    pub fn build(self) -> Request {
        let mut out = Vec::with_capacity(160 + self.body.len());
        push(&mut out, self.method);
        out.push(b' ');
        push(&mut out, self.target);
        push(&mut out, " HTTP/1.1\r\nHost: ");
        push(&mut out, self.host);
        push(&mut out, "\r\nUser-Agent: ");
        push(&mut out, self.user_agent);
        push(&mut out, "\r\nAccept: ");
        push(&mut out, self.accept);
        if let Some(ct) = self.content_type {
            push(&mut out, "\r\nContent-Type: ");
            push(&mut out, ct);
        }
        // A POST states its length even when the body is empty, or the server
        // waits for one that is never coming.
        if !self.body.is_empty() || self.content_type.is_some() {
            push(&mut out, "\r\nContent-Length: ");
            decimal(&mut out, self.body.len());
        }
        push(&mut out, "\r\nConnection: close\r\n\r\n");
        out.extend_from_slice(self.body);
        Request { bytes: out }
    }
}
