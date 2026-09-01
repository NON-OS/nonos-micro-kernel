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

//! The four profiles. Meaning colours come from the system's canonical eight
//! on the dark grounds; the light ground darkens the same hues rather than
//! inventing new ones, because a pale accent on white carries no signal.

use super::types::Theme;

pub const DARK: Theme = Theme {
    bg: 0xFF07_090B, fg: 0xFFD6_DCE3, accent: 0xFF5A_E6D0, path: 0xFF7C_F0A8,
    dim: 0xFF56_5E69, ok: 0xFF35_D07A, warn: 0xFFE5_C07B, err: 0xFFE5_484D,
    chrome_edge: 0xFF1A_1F24,
};

pub const DIM: Theme = Theme {
    bg: 0xB00B_0E12, fg: 0xFFE2_E8EF, accent: 0xFF5A_E6D0, path: 0xFF7C_F0A8,
    dim: 0xFF6B_747E, ok: 0xFF35_D07A, warn: 0xFFE5_C07B, err: 0xFFE5_484D,
    chrome_edge: 0xFF23_2A31,
};

pub const LIGHT: Theme = Theme {
    bg: 0xFFF2_F4F7, fg: 0xFF15_181C, accent: 0xFF0E_7A70, path: 0xFF15_694A,
    dim: 0xFF6A_727C, ok: 0xFF12_784A, warn: 0xFF8A_5B0C, err: 0xFFB0_1218,
    chrome_edge: 0xFFD8_DDE4,
};

pub const ABYSS: Theme = Theme {
    bg: 0xFF00_0308, fg: 0xFFC8_D2DC, accent: 0xFF5A_E6D0, path: 0xFF7C_F0A8,
    dim: 0xFF4A_525B, ok: 0xFF35_D07A, warn: 0xFFE5_C07B, err: 0xFFE5_484D,
    chrome_edge: 0xFF0A_0E13,
};

pub const COUNT: u16 = 4;

pub fn by_index(i: u16) -> &'static Theme {
    match i {
        1 => &DIM,
        2 => &LIGHT,
        3 => &ABYSS,
        _ => &DARK,
    }
}

pub fn by_name(n: &[u8]) -> Option<u16> {
    match n {
        b"dark" | b"blackarch" | b"black" | b"blue" | b"matrix" => Some(0),
        b"dim" | b"glass" | b"smoke" | b"clear" => Some(1),
        b"light" => Some(2),
        b"abyss" => Some(3),
        _ => None,
    }
}
