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

// Closing the fd on drop: best-effort OP_CLOSE so the service releases the
// handle. A failed close has nowhere to report, so the result is ignored.

use super::File;
use crate::sys::fs::nonos::transport::{OP_CLOSE, call};

impl Drop for File {
    fn drop(&mut self) {
        let mut body = [0u8; 8];
        body[..4].copy_from_slice(&self.pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        let _ = call(self.port, OP_CLOSE, &body, 0);
    }
}
