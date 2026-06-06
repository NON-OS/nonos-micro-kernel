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
pub enum Kind {
    Stream,
    Datagram,
    Mixnet,
}

#[derive(Clone, Copy)]
pub struct SocketKey {
    pub pid: u32,
    pub handle: u32,
}

#[derive(Clone, Copy)]
pub struct LocalAddr4 {
    pub port: u16,
}

#[derive(Clone, Copy)]
pub struct RemoteAddr4 {
    pub ip: [u8; 4],
    pub port: u16,
}
