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

pub struct AudioInfo {
    pub rate: u32,
    pub channels: u8,
    pub total_frames: Option<u64>,
}

pub trait Decoder {
    fn info(&self) -> AudioInfo;
    fn next(&mut self, out: &mut [i16]) -> usize;
    fn seek(&mut self, _frame: u64) -> bool {
        false
    }
}
