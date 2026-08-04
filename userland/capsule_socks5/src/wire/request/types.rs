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

/// The destination host of a CONNECT, borrowing the request bytes. A domain
/// name is passed through unresolved so the exit does the lookup: resolving
/// locally would leak the destination to the local network.
pub enum Host<'a> {
    V4([u8; 4]),
    V6([u8; 16]),
    Domain(&'a [u8]),
}

/// A parsed CONNECT request.
pub struct Connect<'a> {
    pub host: Host<'a>,
    pub port: u16,
}

/// The outcome of parsing a request buffer.
pub enum Parsed<'a> {
    /// Not enough bytes yet; read more and try again.
    Incomplete,
    /// Reject with this `REP_*` code, then close.
    Rejected(u8),
    /// A well-formed CONNECT to this destination.
    Connect(Connect<'a>),
}
