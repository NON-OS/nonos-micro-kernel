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

pub const DIR: &[u8] = b"/games/snake";
pub const RANKS: &[u8] = b"/games/snake/ranks.dat";
pub const AWARDS: &[u8] = b"/games/snake/awards.dat";

// Both records are fixed width and capped: ranks tops out at 128 bytes and
// awards at 136, so anything past this bound is not a record this build wrote.
pub const MAX_FILE_BYTES: u32 = 512;
