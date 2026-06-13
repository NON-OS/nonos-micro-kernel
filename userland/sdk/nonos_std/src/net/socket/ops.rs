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

use alloc::vec::Vec;

use super::{read_u32, Socket};
use crate::io::Result;
use crate::net::proto::{
    call, BODY_OFF, OP_ACCEPT, OP_BIND, OP_CONNECT, OP_LISTEN, OP_RECV, OP_SEND,
};

const MAX_PAYLOAD: usize = 1536;

impl Socket {
    pub(crate) fn connect(&self, ip: [u8; 4], port: u16) -> Result<()> {
        call(self.port, OP_CONNECT, &endpoint(self.handle, ip, port), 0).map(|_| ())
    }

    pub(crate) fn bind(&self, ip: [u8; 4], port: u16) -> Result<()> {
        call(self.port, OP_BIND, &endpoint(self.handle, ip, port), 0).map(|_| ())
    }

    pub(crate) fn listen(&self) -> Result<()> {
        call(self.port, OP_LISTEN, &self.handle.to_le_bytes(), 0).map(|_| ())
    }

    pub(crate) fn accept(&self) -> Result<Socket> {
        let rx = call(self.port, OP_ACCEPT, &self.handle.to_le_bytes(), 8)?;
        Ok(Socket { port: self.port, handle: read_u32(&rx, BODY_OFF) })
    }

    pub(crate) fn send(&self, buf: &[u8]) -> Result<usize> {
        let n = buf.len().min(MAX_PAYLOAD);
        let mut body = Vec::with_capacity(4 + n);
        body.extend_from_slice(&self.handle.to_le_bytes());
        body.extend_from_slice(&buf[..n]);
        call(self.port, OP_SEND, &body, 0)?;
        Ok(n)
    }

    pub(crate) fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        let rx = call(self.port, OP_RECV, &self.handle.to_le_bytes(), MAX_PAYLOAD)?;
        let payload = &rx[BODY_OFF..];
        let n = payload.len().min(buf.len());
        buf[..n].copy_from_slice(&payload[..n]);
        Ok(n)
    }
}

fn endpoint(handle: u32, ip: [u8; 4], port: u16) -> [u8; 10] {
    let mut body = [0u8; 10];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&ip);
    body[8..10].copy_from_slice(&port.to_le_bytes());
    body
}
