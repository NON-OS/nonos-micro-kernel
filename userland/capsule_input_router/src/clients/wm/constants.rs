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

pub(super) const MAGIC: u32 = 0x4E57_4D50;
pub(super) const OP_QUERY_FOCUS: u16 = 0x000D;
pub(super) const OP_QUERY_TOPMOST: u16 = 0x000B;
pub(super) const OP_ROUTE_FOCUS: u16 = 0x000C;
pub(super) const SERVICE: &[u8] = b"wm";
