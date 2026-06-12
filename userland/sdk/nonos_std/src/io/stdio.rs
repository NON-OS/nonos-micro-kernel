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

use super::error::{Error, ErrorKind, Result};
use super::write::Write;

pub struct Stdout;
pub struct Stderr;

pub fn stdout() -> Stdout {
    Stdout
}

pub fn stderr() -> Stderr {
    Stderr
}

fn emit(buf: &[u8]) -> Result<usize> {
    if buf.is_empty() {
        return Ok(0);
    }
    let rc = nonos_libc::mk_debug(buf.as_ptr(), buf.len());
    if rc < 0 {
        return Err(Error::new(ErrorKind::Other, "stdout write failed"));
    }
    Ok(buf.len())
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        emit(buf)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        emit(buf)
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
