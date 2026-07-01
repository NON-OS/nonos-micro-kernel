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

pub const MAGIC: u32 = 0x314D_584E;
pub const VERSION: u8 = 1;
pub const HEADER: usize = 16;
pub const MAX_PACKET: usize = 1024;
pub const MAX_BODY: usize = MAX_PACKET - HEADER;

pub struct Frame<'a> {
    pub ip: [u8; 4],
    pub port: u16,
    pub body: &'a [u8],
}
