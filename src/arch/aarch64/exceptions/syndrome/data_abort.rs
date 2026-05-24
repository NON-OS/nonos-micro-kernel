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

#[derive(Debug, Clone, Copy)]
pub struct DataAbortInfo {
    pub wnr: bool,
    pub dfsc: u8,
    pub cm: bool,
    pub s1ptw: bool,
    pub isv: bool,
    pub sas: u8,
    pub sse: bool,
    pub srt: u8,
    pub sf: bool,
    pub ar: bool,
}

pub fn decode_data_abort(iss: u32) -> DataAbortInfo {
    DataAbortInfo {
        wnr: (iss & (1 << 6)) != 0,
        dfsc: (iss & 0x3F) as u8,
        cm: (iss & (1 << 8)) != 0,
        s1ptw: (iss & (1 << 7)) != 0,
        isv: (iss & (1 << 24)) != 0,
        sas: ((iss >> 22) & 0x3) as u8,
        sse: (iss & (1 << 21)) != 0,
        srt: ((iss >> 16) & 0x1F) as u8,
        sf: (iss & (1 << 15)) != 0,
        ar: (iss & (1 << 14)) != 0,
    }
}
