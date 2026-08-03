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
//! One ref a push asks the receiver to move.

use crate::oid::ObjectId;

/// Move `name` from `old` to `new`.
///
/// `old` is what the pusher believes the receiver holds. The receiver refuses
/// the update if it holds something else, which is what stops a push silently
/// overwriting work that arrived in between. A zero id means the ref is
/// expected not to exist yet.
pub struct RefUpdate<'a> {
    pub old: ObjectId,
    pub new: ObjectId,
    /// Full ref name, such as `refs/heads/main`.
    pub name: &'a str,
}
