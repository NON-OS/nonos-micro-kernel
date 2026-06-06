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

use super::super::wire::NIC_HDR_LEN;

pub const MAX_FRAME: usize = 1514;
pub(super) const PREFIX_LEN: usize = 4 + 4;
pub(super) const RESP_CAP: usize = NIC_HDR_LEN + PREFIX_LEN + 12 + MAX_FRAME;
