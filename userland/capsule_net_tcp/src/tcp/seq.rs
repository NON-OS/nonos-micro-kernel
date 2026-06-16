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

pub fn lt(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) < 0
}

pub fn leq(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) <= 0
}

pub fn gt(a: u32, b: u32) -> bool {
    lt(b, a)
}

pub fn geq(a: u32, b: u32) -> bool {
    leq(b, a)
}

pub fn between(x: u32, lo: u32, hi: u32) -> bool {
    leq(lo, x) && lt(x, hi)
}
