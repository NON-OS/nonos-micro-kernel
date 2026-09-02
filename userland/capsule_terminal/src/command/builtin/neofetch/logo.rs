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

//! The compact NONOS emblem: a rounded frame with the Ø slash through it,
//! narrow enough that the info column still fits beside it in 80 columns.

/// Display width of every [`LOGO`] row, in columns.
///
/// The rows are box-drawing characters, so this is deliberately a character
/// count and never a byte count — `str::len` on these rows is three times
/// too large and would push the info column off the right edge.
pub const LOGO_W: usize = 16;

pub const LOGO: [&str; 8] = [
    "                ",
    " ╭────────────╮ ",
    " │          ╱ │ ",
    " │        ╱   │ ",
    " │      ╱     │ ",
    " │    ╱       │ ",
    " │  ╱         │ ",
    " ╰────────────╯ ",
];
