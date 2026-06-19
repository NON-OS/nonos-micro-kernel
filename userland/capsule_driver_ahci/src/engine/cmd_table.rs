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

use super::prdt::PrdtEntry;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CmdTable {
    pub cfis: [u8; 64],
    pub acmd: [u8; 16],
    pub rsv: [u8; 48],
    pub prdt: [PrdtEntry; 1],
}

const _: () = assert!(core::mem::size_of::<CmdTable>() == 144);
pub const PRDT_OFFSET: usize = 128;
