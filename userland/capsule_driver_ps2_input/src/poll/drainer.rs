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
pub struct Drainer {
    pub(super) pending_e0: bool,
    pub(super) pending_e1: bool,
    pub(super) mods: u16,
    // Caps lock is a toggle, not a held modifier, so it lives beside the
    // modifier bits and is folded into the published flags per event.
    pub(super) caps: bool,
}
impl Drainer {
    pub const fn new() -> Self {
        Self { pending_e0: false, pending_e1: false, mods: 0, caps: false }
    }
}
