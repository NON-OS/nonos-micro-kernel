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

use crate::term::grid::cell::{F_BOLD, F_REVERSE, F_UNDERLINE};
use crate::term::grid::types::Grid;
use crate::term::vt::color::{ansi_to_argb, pack_rgb, DEFAULT_BG, DEFAULT_FG};

pub fn sgr(g: &mut Grid, params: &[i64]) {
    if params.is_empty() {
        g.fg = DEFAULT_FG;
        g.bg = DEFAULT_BG;
        g.flags = 0;
        return;
    }
    let mut i = 0;
    while i < params.len() {
        let v = params[i];
        match v {
            0 => {
                g.fg = DEFAULT_FG;
                g.bg = DEFAULT_BG;
                g.flags = 0;
            }
            1 => {
                g.flags |= F_BOLD;
            }
            4 => {
                g.flags |= F_UNDERLINE;
            }
            7 => {
                g.flags |= F_REVERSE;
            }
            21 | 22 => {
                g.flags &= !F_BOLD;
            }
            24 => {
                g.flags &= !F_UNDERLINE;
            }
            27 => {
                g.flags &= !F_REVERSE;
            }
            30..=37 => {
                g.fg = ansi_to_argb((v - 30) as u8);
            }
            38 => {
                i += ext_color(params, i, &mut g.fg);
            }
            39 => {
                g.fg = DEFAULT_FG;
            }
            40..=47 => {
                g.bg = ansi_to_argb((v - 40) as u8);
            }
            48 => {
                i += ext_color(params, i, &mut g.bg);
            }
            49 => {
                g.bg = DEFAULT_BG;
            }
            90..=97 => {
                g.fg = ansi_to_argb((v - 90 + 8) as u8);
            }
            100..=107 => {
                g.bg = ansi_to_argb((v - 100 + 8) as u8);
            }
            _ => {}
        }
        i += 1;
    }
}

// Extended colour after 38/48: `5;n` is a 256-colour index, `2;r;g;b` is true
// 24-bit colour kept verbatim. Returns how many extra params were consumed.
fn ext_color(params: &[i64], i: usize, slot: &mut u32) -> usize {
    match params.get(i + 1).copied() {
        Some(5) => {
            *slot = ansi_to_argb(params.get(i + 2).copied().unwrap_or(0) as u8);
            2
        }
        Some(2) => {
            let r = params.get(i + 2).copied().unwrap_or(0) as u8;
            let gg = params.get(i + 3).copied().unwrap_or(0) as u8;
            let b = params.get(i + 4).copied().unwrap_or(0) as u8;
            *slot = pack_rgb(r, gg, b);
            4
        }
        _ => 0,
    }
}
