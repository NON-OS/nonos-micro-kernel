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
//! The git transport, over TLS.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;

use nonos_git::{Transport, TransportError};
use nonos_http::RequestBuilder;

use super::round_trip::round_trip;
use super::url::Remote;

pub struct Https {
    pub(super) remote: Remote,
    pub(super) now: u64,
}

impl Https {
    /// `now` is the wall clock the certificate is judged against. A caller
    /// that does not know the time passes zero, and every certificate then
    /// reads as expired, which is the safe direction to fail.
    pub fn new(remote: Remote, now: u64) -> Https {
        Https { remote, now }
    }
}

impl Transport for Https {
    fn get(&mut self, path: &str) -> Result<Vec<u8>, TransportError> {
        let target = format!("{}{}", self.remote.base, path);
        let request = RequestBuilder::get(&self.remote.host, &target).build();
        round_trip(self, request.bytes)
    }

    fn post(
        &mut self,
        path: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        let target = format!("{}{}", self.remote.base, path);
        let request = RequestBuilder::post(&self.remote.host, &target, content_type, body).build();
        round_trip(self, request.bytes)
    }
}
