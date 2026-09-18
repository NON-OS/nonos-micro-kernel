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


//! What the serve loop does with a trap.

/// A parked caller is not an error: the guest thread stays inside its
/// syscall with no reply until something else here wakes it. That is how
/// a futex wait works, and it is why the loop cannot reply to every
/// frame it takes.
pub enum Answer {
    Reply(u64),
    Park,
}

impl Answer {
    pub fn value(v: u64) -> Answer {
        Answer::Reply(v)
    }
}
