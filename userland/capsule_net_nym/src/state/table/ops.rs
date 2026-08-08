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

use super::super::{Gateway, Session};
use super::topology_gate;
use super::types::{Table, TableError, TABLE_CAP};
use crate::crypto::Key;
use crate::state;

impl Table {
    /// Bind a gateway, dropping sessions only when it is a different one.
    ///
    /// Sessions are keyed to the gateway that carries them, so a change has
    /// to clear them. Rebinding the same gateway does not, and clearing
    /// regardless meant a client that opened a session could have it taken
    /// away by a reconnect it never asked for.
    pub fn set_gateway(&mut self, gateway: Gateway) -> Option<Gateway> {
        let changed = match self.gateway {
            Some(current) => current.ip != gateway.ip || current.port != gateway.port,
            None => false,
        };
        if changed {
            self.reset_sessions();
        }
        self.gateway.replace(gateway)
    }

    pub fn open(&mut self, owner: u32, key: Key) -> Result<u32, TableError> {
        if self.sessions.len() >= TABLE_CAP {
            return Err(TableError::Full);
        }
        topology_gate::check()?;
        let gateway = self.gateway.ok_or(TableError::NoGateway)?;
        let id = self.alloc_id();
        self.sessions.push(Session::new(owner, id, gateway, key));
        Ok(id)
    }

    pub fn with_mut<R>(
        &mut self,
        owner: u32,
        id: u32,
        f: impl FnOnce(&mut Session) -> R,
    ) -> Option<R> {
        self.sessions.iter_mut().find(|s| s.owner == owner && s.id == id).map(f)
    }

    pub fn with_id_mut<R>(&mut self, id: u32, f: impl FnOnce(&mut Session) -> R) -> Option<R> {
        self.sessions.iter_mut().find(|s| s.id == id).map(f)
    }

    pub fn close(&mut self, owner: u32, id: u32) -> bool {
        let Some(pos) = self.sessions.iter().position(|s| s.owner == owner && s.id == id) else {
            return false;
        };
        let mut session = self.sessions.remove(pos);
        session.zeroize();
        true
    }

    /// Whether any session is open. A directory fetch takes a TLS handshake
    /// and a large response, and this capsule answers nobody while it runs,
    /// so it waits until no client is depending on it.
    pub fn idle(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn gateway(&self) -> Option<Gateway> {
        self.gateway
    }

    fn alloc_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1).max(1);
        id
    }
}
