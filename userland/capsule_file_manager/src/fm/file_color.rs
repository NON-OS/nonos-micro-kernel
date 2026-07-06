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

use super::filetype::Kind;

pub fn color(kind: Kind) -> u32 {
    match kind {
        Kind::Dir => 0xFF6CE08C,
        Kind::Code => 0xFF7FB4FF,
        Kind::Image => 0xFFD08CF0,
        Kind::Doc => 0xFFD7E2F2,
        Kind::Archive => 0xFFE0B060,
        Kind::Exec => 0xFFE0785C,
        Kind::Other => 0xFFA8B6CC,
    }
}
