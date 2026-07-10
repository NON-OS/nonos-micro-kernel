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

// Outcome of every not-yet-wired backend call. The UI renders each variant
// honestly; a stub returns NotWired, a live provider returns Pending then Ready.
#[derive(Clone)]
pub enum Seam<T> {
    NotWired,
    Pending,
    Ready(T),
    Failed(&'static str),
}

impl<T> Seam<T> {
    pub fn is_ready(&self) -> bool {
        matches!(self, Seam::Ready(_))
    }
    pub fn as_ready(&self) -> Option<&T> {
        match self {
            Seam::Ready(v) => Some(v),
            _ => None,
        }
    }
}

// Association-set membership verdict for a shielded note.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Inclusion {
    Pending,
    Included,
    Excluded,
    NotWired,
}
