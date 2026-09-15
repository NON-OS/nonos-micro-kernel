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

//! A software TPM 2.0 on a socket, for the life of one test.
//!
//! `swtpm socket --tpm2` speaks raw command bytes on its server socket, which
//! is exactly what the kernel puts through the CRB window, so the same
//! `Vec<u8>` the builders produce is what gets sent.
//!
//! Unix sockets rather than TCP ports. Each test needs its own TPM, because
//! each mutates PCR state, and six of them start at once under the test
//! harness: allocating two free ports per instance raced, and the loser saw
//! its connection closed rather than a response. A socket in a directory
//! named after the process cannot collide. The path stays short because the
//! sockaddr caps at 104 bytes and overruns it silently.

use std::os::unix::net::UnixStream;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use super::swtpm_io::tempdir;

pub struct Swtpm {
    pub(super) child: Child,
    pub(super) stream: UnixStream,
    pub(super) _state: tempdir::Dir,
}

impl Swtpm {
    /// `None` when swtpm is not installed. The live tests then pass as
    /// skipped, and say so, rather than pretending to a result.
    pub fn start() -> Option<Self> {
        let state = tempdir::Dir::new();
        let sock = state.path().join("s");
        let ctrl = state.path().join("c");
        let child = Command::new("swtpm")
            .args(["socket", "--tpm2"])
            .arg(format!("--tpmstate=dir={}", state.path().display()))
            .arg(format!("--server=type=unixio,path={}", sock.display()))
            .arg(format!("--ctrl=type=unixio,path={}", ctrl.display()))
            .arg("--flags=not-need-init,startup-clear")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Ok(stream) = UnixStream::connect(&sock) {
                return Some(Swtpm { child, stream, _state: state });
            }
            if Instant::now() >= deadline {
                let mut dead = child;
                let _ = dead.kill();
                let _ = dead.wait();
                return None;
            }
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}
