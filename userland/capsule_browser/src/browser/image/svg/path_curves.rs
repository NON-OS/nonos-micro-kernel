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

use super::arc::arc_to;
use super::curves::{cubic_to, quad_to};
use super::path_state::PathState;
use super::path_tok::Tok;

type P = [f32; 2];

// Reflect `anchor` through the pen for the smooth S/T commands; the pen
// itself when the previous command was not of the matching curve family.
fn reflect(st: &PathState, anchor: Option<P>) -> P {
    match anchor {
        Some(a) => [2.0 * st.cur[0] - a[0], 2.0 * st.cur[1] - a[1]],
        None => st.cur,
    }
}

// One C/S argument group: two control points (the first reflected for S).
pub(super) fn cubic(st: &mut PathState, tk: &mut Tok, rel: bool, smooth: bool) -> Option<()> {
    let c1 = if smooth { reflect(st, st.last_c2) } else { tk.xy(st.cur, rel)? };
    let c2 = tk.xy(st.cur, rel)?;
    let p1 = tk.xy(st.cur, rel)?;
    cubic_to(&mut st.sub, st.cur, c1, c2, p1);
    st.cur = p1;
    st.last_c2 = Some(c2);
    st.last_q = None;
    Some(())
}

// One Q/T argument group.
pub(super) fn quad(st: &mut PathState, tk: &mut Tok, rel: bool, smooth: bool) -> Option<()> {
    let c = if smooth { reflect(st, st.last_q) } else { tk.xy(st.cur, rel)? };
    let p1 = tk.xy(st.cur, rel)?;
    quad_to(&mut st.sub, st.cur, c, p1);
    st.cur = p1;
    st.last_q = Some(c);
    st.last_c2 = None;
    Some(())
}

// One A argument group: radii, rotation, the two flags, endpoint.
pub(super) fn arc(st: &mut PathState, tk: &mut Tok, rel: bool) -> Option<()> {
    let rx = tk.num()?;
    let ry = tk.num()?;
    let rot = tk.num()?;
    let large = tk.flag()?;
    let sweep = tk.flag()?;
    let p1 = tk.xy(st.cur, rel)?;
    arc_to(&mut st.sub, st.cur, [rx, ry], rot, large, sweep, p1);
    st.cur = p1;
    st.last_c2 = None;
    st.last_q = None;
    Some(())
}
