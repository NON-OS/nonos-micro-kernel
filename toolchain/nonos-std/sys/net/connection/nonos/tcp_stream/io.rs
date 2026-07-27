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

// TcpStream reads and writes. Each carries the configured deadline (or 1 ms
// when nonblocking) into the IPC call and maps a lapse to WouldBlock in
// nonblocking mode. peek is unsupported (the net stack has no peek yet).

use super::TcpStream;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::sys::net::connection::nonos::transport::{BODY, MAX_PAYLOAD, OP_RECV, OP_SEND, sk_timed};
use crate::sys::unsupported;
use crate::vec::Vec;
use core::sync::atomic::Ordering::Relaxed;

impl TcpStream {
    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let handle = self.socket.handle()?;
        let nb = self.nonblocking.load(Relaxed);
        let t = if nb { 1 } else { self.read_timeout_ms.load(Relaxed) };
        let rx = match sk_timed(OP_RECV, &handle.to_le_bytes(), MAX_PAYLOAD, t) {
            Ok(rx) => rx,
            Err(e) if nb && e.kind() == io::ErrorKind::TimedOut => {
                return Err(io::Error::from(io::ErrorKind::WouldBlock));
            }
            Err(e) => return Err(e),
        };
        let data = &rx[BODY..];
        let n = data.len().min(buf.len());
        buf[..n].copy_from_slice(&data[..n]);
        Ok(n)
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let mut tmp = [0u8; 1536];
        let n = self.read(&mut tmp)?;
        cursor.append(&tmp[..n]);
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.read(b);
            }
        }
        Ok(0)
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let handle = self.socket.handle()?;
        let nb = self.nonblocking.load(Relaxed);
        let t = if nb { 1 } else { self.write_timeout_ms.load(Relaxed) };
        let n = buf.len().min(MAX_PAYLOAD);
        let mut body = Vec::with_capacity(4 + n);
        body.extend_from_slice(&handle.to_le_bytes());
        body.extend_from_slice(&buf[..n]);
        match sk_timed(OP_SEND, &body, 0, t) {
            Ok(_) => Ok(n),
            Err(e) if nb && e.kind() == io::ErrorKind::TimedOut => {
                Err(io::Error::from(io::ErrorKind::WouldBlock))
            }
            Err(e) => Err(e),
        }
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.write(b);
            }
        }
        Ok(0)
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }
}
