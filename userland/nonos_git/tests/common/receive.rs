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
//! Running real `git receive-pack` against a push we built.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Feed `body` to a receive-pack serving `bare` and return what it answered.
pub fn receive_pack(bare: &Path, body: &[u8]) -> String {
    let mut child = Command::new("git")
        .arg("receive-pack")
        .arg(bare)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("spawning receive-pack: {e}"));

    let mut stdin = child.stdin.take().expect("stdin");
    // Receive-pack advertises its refs first and does not read until it has
    // written them, so the write happens on another thread to avoid a stall on
    // a full pipe.
    let owned = body.to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&owned));
    let out = child.wait_with_output().expect("receive-pack output");
    writer.join().expect("writer thread").expect("writing push body");

    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}
