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
pub struct TrafficKeys {
    pub suite: u16,
    pub handshake_secret: [u8; 32],
    pub client_secret: [u8; 32],
    pub server_secret: [u8; 32],
    pub client_key: [u8; 32],
    pub client_iv: [u8; 12],
    pub server_key: [u8; 32],
    pub server_iv: [u8; 12],
}
