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
pub struct Context {
    pub keyring_port: u32,
    pub desktop_shell_port: u32,
    pub compositor_port: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub backing_va: u64,
    pub(in crate::state::context) serial: u32,
    pub(in crate::state::context) state: SessionState,
}

pub(in crate::state::context) enum SessionState {
    Locked,
    Unlocked { owner_pid: u32, key_id: u32, serial: u32 },
}
