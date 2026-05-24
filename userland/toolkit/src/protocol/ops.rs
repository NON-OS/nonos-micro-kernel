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

pub const TOOLKIT_ENDPOINT: u64 = 4610;

pub const TOOLKIT_OP_HEALTHCHECK: u16 = 0x0000;
pub const TOOLKIT_OP_THEME_APPLY: u16 = 0x0001;
pub const TOOLKIT_OP_ANIMATION_TICK: u16 = 0x0002;
pub const TOOLKIT_OP_COMPONENT_RENDER: u16 = 0x0003;
pub const TOOLKIT_OP_THEME_GET: u16 = 0x0004;

pub const IPC_PAYLOAD_MAX: usize = 256;
pub const THEME_PAYLOAD_LEN: usize = 24;
