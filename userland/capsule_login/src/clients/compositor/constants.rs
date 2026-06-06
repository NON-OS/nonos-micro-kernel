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
pub const MAGIC: u32 = 0x4E43_4D50;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const OP_HEALTHCHECK: u16 = 0x0001;
pub const OP_SCENE_SUBMIT: u16 = 0x0002;
pub const OP_DAMAGE_COMMIT: u16 = 0x0003;
pub const SCENE_REQ_LEN: usize = 32;
pub const DAMAGE_REQ_LEN: usize = 16;
