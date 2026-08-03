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
//! Running the commands and returning what they wrote.

use std::io::Write;
use std::process::{Command, Stdio};

use nonos_git::{Transport, TransportError};

use super::banner::{banner, service_of};
use super::types::LocalGit;

impl Transport for LocalGit {
    fn get(&mut self, path: &str) -> Result<Vec<u8>, TransportError> {
        let service = service_of(path).ok_or(TransportError::Status(404))?;
        let out = Command::new("git")
            .args([service.trim_start_matches("git-"), "--advertise-refs"])
            .arg(&self.dir)
            .output()
            .map_err(|_| TransportError::Unreachable)?;
        if !out.status.success() {
            return Err(TransportError::Status(500));
        }
        let mut body = banner(service);
        body.extend_from_slice(&out.stdout);
        Ok(body)
    }

    fn post(
        &mut self,
        path: &str,
        _content_type: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        let service = service_of(path).ok_or(TransportError::Status(404))?;
        let mut child = Command::new("git")
            .args([service.trim_start_matches("git-"), "--stateless-rpc"])
            .arg(&self.dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| TransportError::Unreachable)?;
        let mut stdin = child.stdin.take().ok_or(TransportError::Closed)?;
        // Writing on another thread: receive-pack advertises before it reads,
        // so a single thread can deadlock on a full pipe.
        let owned = body.to_vec();
        let writer = std::thread::spawn(move || stdin.write_all(&owned));
        let out = child.wait_with_output().map_err(|_| TransportError::Closed)?;
        let _ = writer.join();
        Ok(out.stdout)
    }
}
