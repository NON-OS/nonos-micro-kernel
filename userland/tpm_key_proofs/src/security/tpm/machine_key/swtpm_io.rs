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

//! Talking to the software TPM once it is up, and taking it down again.

use std::io::{Read, Write};

use super::swtpm::Swtpm;

impl Swtpm {
    /// One command, one response. The response size is in its own header.
    pub fn run(&mut self, cmd: &[u8]) -> Vec<u8> {
        self.stream.write_all(cmd).expect("write to swtpm");
        let mut head = [0u8; 10];
        self.stream.read_exact(&mut head).expect("response header");
        let size = u32::from_be_bytes([head[2], head[3], head[4], head[5]]) as usize;
        let mut resp = head.to_vec();
        resp.resize(size.max(10), 0);
        self.stream.read_exact(&mut resp[10..]).expect("response body");
        resp
    }
}

impl Drop for Swtpm {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// A directory that goes away with the value, without a crate for it.
pub mod tempdir {
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT: AtomicU32 = AtomicU32::new(0);

    pub struct Dir(PathBuf);

    impl Dir {
        pub fn new() -> Self {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let p = PathBuf::from(format!("/tmp/nonos-tpm-{}-{n}", std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(&p).expect("state dir");
            Dir(p)
        }
        pub fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for Dir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
