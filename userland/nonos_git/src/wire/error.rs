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
//! Why a wire response cannot be read.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WireError {
    /// A packet length was not four hex digits.
    Length,
    /// A packet claimed more bytes than the response holds.
    Truncated,
    /// The first packet was not the service banner a smart fetch begins with.
    NotSmartHttp,
    /// A ref line was not `<40 hex> <name>`.
    RefLine,
    /// The server reported an error in place of a ref advertisement.
    Remote,
}
