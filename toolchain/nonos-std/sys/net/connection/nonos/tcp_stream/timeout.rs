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

// Read/write timeouts and nonblocking mode. The values are stored so a getter
// returns what its setter stored; the read/write path in io.rs enforces them
// through the IPC deadline. A zero Duration is rejected, matching std.

use super::TcpStream;
use crate::io;
use crate::sys::net::connection::nonos::transport::{dur_to_ms, ms_to_dur};
use crate::time::Duration;
use core::sync::atomic::Ordering::Relaxed;

impl TcpStream {
    pub fn set_read_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        if matches!(t, Some(d) if d.is_zero()) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "zero timeout"));
        }
        self.read_timeout_ms.store(dur_to_ms(t), Relaxed);
        Ok(())
    }

    pub fn set_write_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        if matches!(t, Some(d) if d.is_zero()) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "zero timeout"));
        }
        self.write_timeout_ms.store(dur_to_ms(t), Relaxed);
        Ok(())
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(ms_to_dur(self.read_timeout_ms.load(Relaxed)))
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(ms_to_dur(self.write_timeout_ms.load(Relaxed)))
    }

    pub fn set_nonblocking(&self, on: bool) -> io::Result<()> {
        self.nonblocking.store(on, Relaxed);
        Ok(())
    }
}
