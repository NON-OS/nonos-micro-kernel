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

use super::wire::exchange;

const VER: u8 = 0x05;
const METHOD_NONE: u8 = 0x00;
const CMD_CONNECT: u8 = 0x01;
const ATYP_DOMAIN: u8 = 0x03;
const REP_OK: u8 = 0x00;

/// A stream carried by `net.socks5`, which reaches the host through the
/// mixnet. Bytes buffered here are what the proxy has already answered.
pub struct SocksStream {
    port: u32,
    pending: Vec<u8>,
}

impl SocksStream {
    /// Greet the proxy and ask it to reach `host:port`.
    ///
    /// The name goes to the exit unresolved, so the destination is never
    /// looked up from this machine.
    pub fn connect(port: u32, host: &str, dst_port: u16) -> Result<Self, ()> {
        let greeting = [VER, 1, METHOD_NONE];
        let reply = exchange(port, &greeting)?;
        if reply.len() < 2 || reply[0] != VER || reply[1] != METHOD_NONE {
            return Err(());
        }
        let name = host.as_bytes();
        if name.is_empty() || name.len() > 255 {
            return Err(());
        }
        let mut req = Vec::with_capacity(7 + name.len());
        req.extend_from_slice(&[VER, CMD_CONNECT, 0, ATYP_DOMAIN, name.len() as u8]);
        req.extend_from_slice(name);
        req.extend_from_slice(&dst_port.to_be_bytes());
        let reply = exchange(port, &req)?;
        if reply.len() < 2 || reply[1] != REP_OK {
            return Err(());
        }
        Ok(Self { port, pending: Vec::new() })
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), ()> {
        let reply = exchange(self.port, data)?;
        self.pending.extend_from_slice(&reply);
        Ok(())
    }

    /// Take what the proxy has already answered. Zero means nothing waiting.
    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, ()> {
        let n = into.len().min(self.pending.len());
        into[..n].copy_from_slice(&self.pending[..n]);
        self.pending.drain(..n);
        Ok(n)
    }
}
