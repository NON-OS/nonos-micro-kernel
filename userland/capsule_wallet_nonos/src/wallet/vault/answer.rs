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

/// What came back.
///
/// A server that did not answer and a server that answered "no such file" are
/// different facts, and collapsing them cost the restore path its retry: the
/// store is deaf while it stages packages at boot, which is exactly when a
/// window asks whether a wallet is stored, and a silent no reads as an empty
/// disk. The caller decides what to do with each.
pub(super) enum Answer {
    /// Reply received, status 0, this many bytes.
    Ok(usize),
    /// Reply received with a failure status. The server is up and this is its
    /// answer.
    Refused,
    /// Nothing came back inside the timeout. Says nothing about the disk.
    Silent,
}

impl Answer {
    /// For callers that only need to know whether it worked: a save either
    /// landed or it did not, and both failures mean the same thing there.
    pub(super) fn worked(&self) -> bool {
        matches!(self, Answer::Ok(_))
    }

    pub(super) fn len(&self) -> Option<usize> {
        match self {
            Answer::Ok(n) => Some(*n),
            _ => None,
        }
    }
}
