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
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Trb {
    pub d0: u32,
    pub d1: u32,
    pub d2: u32,
    pub d3: u32,
}
impl Trb {
    pub const fn zero() -> Self {
        Self { d0: 0, d1: 0, d2: 0, d3: 0 }
    }
}