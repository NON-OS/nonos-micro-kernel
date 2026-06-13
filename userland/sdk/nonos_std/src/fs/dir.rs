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

use alloc::string::String;
use alloc::vec::Vec;

use super::frame::{BODY_OFF, OP_LIST, OP_MKDIR, OP_UNLINK};
use super::session::{call, connect, path_body};
use crate::io::Result;

pub struct DirEntry {
    name: String,
}

impl DirEntry {
    pub fn file_name(&self) -> String {
        self.name.clone()
    }

    pub fn is_dir(&self) -> bool {
        self.name.ends_with('/')
    }
}

pub fn read_dir(path: &[u8]) -> Result<Vec<DirEntry>> {
    let vfs = connect()?;
    let rx = call(vfs.port, OP_LIST, &path_body(vfs.pid, path)?, 65536)?;
    Ok(parse_entries(&rx[BODY_OFF..]))
}

pub fn create_dir(path: &[u8]) -> Result<()> {
    let vfs = connect()?;
    call(vfs.port, OP_MKDIR, &path_body(vfs.pid, path)?, 0).map(|_| ())
}

pub fn remove_file(path: &[u8]) -> Result<()> {
    let vfs = connect()?;
    call(vfs.port, OP_UNLINK, &path_body(vfs.pid, path)?, 0).map(|_| ())
}

fn parse_entries(mut body: &[u8]) -> Vec<DirEntry> {
    let mut out = Vec::new();
    while !body.is_empty() {
        let len = body[0] as usize;
        if body.len() < 1 + len {
            break;
        }
        if let Ok(name) = core::str::from_utf8(&body[1..1 + len]) {
            out.push(DirEntry { name: String::from(name) });
        }
        body = &body[1 + len..];
    }
    out
}
