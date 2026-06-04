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
use super::{Context, SessionState};

impl Context {
    pub fn new(
        keyring_port: u32,
        desktop_shell_port: u32,
        compositor_port: u32,
        width: u32,
        height: u32,
        stride: u32,
        backing_va: u64,
    ) -> Self {
        Self {
            keyring_port,
            desktop_shell_port,
            compositor_port,
            width,
            height,
            stride,
            backing_va,
            serial: 0,
            state: SessionState::Locked,
        }
    }
}
