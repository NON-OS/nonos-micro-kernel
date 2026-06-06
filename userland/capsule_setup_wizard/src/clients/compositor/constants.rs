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

pub(super) const DAMAGE_REQ_LEN: usize = 16;
pub(super) const HDR_LEN: usize = 20;
pub(super) const MAGIC: u32 = 0x4E43_4D50;
pub(super) const OP_DAMAGE_COMMIT: u16 = 0x0003;
pub(super) const OP_HEALTHCHECK: u16 = 0x0001;
pub(super) const OP_SCENE_REMOVE: u16 = 0x0007;
pub(super) const OP_SCENE_SUBMIT: u16 = 0x0002;
pub(super) const SCENE_REMOVE_REQ_LEN: usize = 8;
pub(super) const SCENE_REQ_LEN: usize = 32;
pub(super) const VERSION: u16 = 1;
