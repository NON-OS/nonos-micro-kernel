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
//! The all-zero id.

use super::object_id::ObjectId;

impl ObjectId {
    /// The id git uses to mean "this ref does not exist yet".
    ///
    /// It is not a hash of anything. A push naming it as the value it expects
    /// is saying the branch is unborn, and the receiver refuses the push if
    /// the branch turns out to exist after all.
    pub fn zero() -> ObjectId {
        ObjectId::from_bytes([0u8; 20])
    }
}
