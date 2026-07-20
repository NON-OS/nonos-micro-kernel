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

// File::truncate. The protocol has no by-fd truncate, so it goes by the
// path captured at open, carrying the process id and the new size.

use super::File;
use crate::io;
use crate::sys::fs::nonos::transport::{OP_TRUNCATE, call};
use crate::vec::Vec;

impl File {
    pub fn truncate(&self, size: u64) -> io::Result<()> {
        let mut body = Vec::with_capacity(13 + self.path.len());
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.push(self.path.len() as u8);
        body.extend_from_slice(&self.path);
        body.extend_from_slice(&size.to_le_bytes());
        call(self.port, OP_TRUNCATE, &body, 0).map(|_| ())
    }
}
