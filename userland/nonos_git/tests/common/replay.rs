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
//! A transport that replays responses captured from a real server.

extern crate alloc;

use nonos_git::{Transport, TransportError};

/// Serves recorded bytes, and records what it was asked for.
///
/// Nothing here invents a response. Both bodies are what github.com sent for
/// octocat/Hello-World, so a test using this exercises the same parsing path
/// as a live fetch, minus the socket.
pub struct Replay {
    pub advert: Vec<u8>,
    pub pack: Vec<u8>,
    pub asked: Vec<String>,
}

impl Replay {
    pub fn new(advert: &[u8], pack: &[u8]) -> Replay {
        Replay { advert: advert.to_vec(), pack: pack.to_vec(), asked: Vec::new() }
    }
}

impl Transport for Replay {
    fn get(&mut self, path: &str) -> Result<Vec<u8>, TransportError> {
        self.asked.push(String::from(path));
        if path.contains("service=git-upload-pack") {
            return Ok(self.advert.clone());
        }
        Err(TransportError::Status(404))
    }

    fn post(
        &mut self,
        path: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        self.asked.push(String::from(path));
        // A server that is sent the wrong content type answers with an error,
        // so the test holds this layer to the same rule.
        if !content_type.starts_with("application/x-git-") {
            return Err(TransportError::Status(415));
        }
        if !body.starts_with(b"0048want ") {
            return Err(TransportError::Status(400));
        }
        Ok(self.pack.clone())
    }
}
