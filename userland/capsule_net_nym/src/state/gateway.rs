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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    RawTcp,
    WebSocket,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Gateway {
    pub ip: [u8; 4],
    pub port: u16,
    pub stream: u32,
    pub transport: Transport,
    /// The gateway's Ed25519 identity from the directory. Zero means none was
    /// supplied and registration is skipped: there would be nothing to
    /// authenticate against.
    pub identity: [u8; 32],
    /// Derived by the handshake; zero until it completes.
    pub shared_key: [u8; 32],
}
