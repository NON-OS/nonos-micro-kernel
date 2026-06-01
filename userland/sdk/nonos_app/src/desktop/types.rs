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

use nonos_desktop::Peers;

pub(crate) struct DesktopWindow {
    pub(crate) handle: u64,
    pub(crate) backing: *mut u32,
    pub(crate) pixels: usize,
    pub(crate) byte_len: u64,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) window_id: u32,
    pub(crate) peers: Peers,
    pub(crate) rid: u32,
}
