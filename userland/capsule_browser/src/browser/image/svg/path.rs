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

use alloc::vec::Vec;

use super::path_curves::{arc, cubic, quad};
use super::path_state::PathState;
use super::path_tok::Tok;

// Path data to flattened subpaths. Implicit command repetition applies, and
// a moveto's extra pairs continue as lineto per the spec. A malformed tail
// stops the walk; whatever parsed before it still renders.
pub(super) fn parse_path(d: &str) -> Vec<Vec<[f32; 2]>> {
    let mut tk = Tok::new(d);
    let mut st = PathState::new();
    let mut cmd = 0u8;
    while !tk.at_end() {
        if let Some(c) = tk.cmd() {
            cmd = c;
        } else if cmd == 0 {
            break;
        }
        let rel = cmd.is_ascii_lowercase();
        let up = cmd.to_ascii_uppercase();
        let step = match up {
            b'M' => {
                let p = tk.xy(st.cur, rel);
                if let Some(p) = p {
                    st.move_to(p);
                    // Further pairs are implicit linetos.
                    cmd = if rel { b'l' } else { b'L' };
                }
                p.map(|_| ())
            }
            b'L' => tk.xy(st.cur, rel).map(|p| {
                st.sub.push(p);
                st.cur = p;
            }),
            b'H' => tk.num().map(|x| {
                let p = [if rel { st.cur[0] + x } else { x }, st.cur[1]];
                st.sub.push(p);
                st.cur = p;
            }),
            b'V' => tk.num().map(|y| {
                let p = [st.cur[0], if rel { st.cur[1] + y } else { y }];
                st.sub.push(p);
                st.cur = p;
            }),
            b'C' => cubic(&mut st, &mut tk, rel, false),
            b'S' => cubic(&mut st, &mut tk, rel, true),
            b'Q' => quad(&mut st, &mut tk, rel, false),
            b'T' => quad(&mut st, &mut tk, rel, true),
            b'A' => arc(&mut st, &mut tk, rel),
            b'Z' => {
                st.close();
                Some(())
            }
            _ => None,
        };
        // Only a curve command carries its reflection anchor forward.
        if !matches!(up, b'C' | b'S') {
            st.last_c2 = None;
        }
        if !matches!(up, b'Q' | b'T') {
            st.last_q = None;
        }
        if step.is_none() {
            break;
        }
    }
    st.finish()
}
