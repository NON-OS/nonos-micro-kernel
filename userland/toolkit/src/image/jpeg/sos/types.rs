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
use crate::image::jpeg::sof0::MAX_COMPS;

#[derive(Clone, Copy)]
pub struct ScanComp {
    pub frame_index: usize,
    pub td: u8,
    pub ta: u8,
}

#[derive(Clone, Copy)]
pub struct ScanHeader {
    pub ns: u8,
    pub comps: [ScanComp; MAX_COMPS],
    pub ss: u8,
    pub se: u8,
    pub ah: u8,
    pub al: u8,
}
