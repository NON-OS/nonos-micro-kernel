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
//! What can go wrong against the sockets capsule.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SocketError {
    /// No net.sockets service is registered, so there is no network at all.
    NoService,
    /// The capsule did not reply, or replied with a frame this refused.
    Protocol,
    /// The capsule answered with a failure status.
    Refused,
    /// The host name is empty or longer than a domain name can be.
    BadHost,
    /// The request body does not fit one frame.
    TooLarge,
}
