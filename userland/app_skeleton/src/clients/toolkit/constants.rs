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

pub(super) const NOTK_MAGIC: u32 = 0x4E4F_544B;
pub(super) const HDR_LEN: usize = 16;
pub(super) const TOOLKIT_OP_COMPONENT_RENDER: u16 = 0x0003;
pub(super) const STATUS_OK: u16 = 0;
pub(super) const KIND_PANEL: u16 = 0;
pub(super) const KIND_LABEL: u16 = 2;
pub(super) const CHROME_H: u32 = 28;
pub(super) const LABEL_X: u32 = 10;
pub(super) const LABEL_Y: u32 = 8;
pub(super) const MAX_LABEL_BYTES: usize = 96;
