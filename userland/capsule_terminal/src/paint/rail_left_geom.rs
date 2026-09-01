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

use super::rail_row::{inside, row_at, row_h};
use super::rail_text::{lh, RAIL_GAP, RAIL_PAD};
use crate::layout::Rect;

pub enum LeftHit {
    NewSession,
    AddProject,
    Session(u32),
    Project(u32),
}

pub struct Sections {
    pub s_head: Rect,
    pub s_plus: Rect,
    pub s_list: Rect,
    pub p_head: Rect,
    pub p_plus: Rect,
    pub p_list: Rect,
}

/// Height `rail_text::head` consumes, so the stack below a caption is derived
/// once instead of guessed by each caller.
pub fn head_h() -> u32 {
    RAIL_GAP + lh() + RAIL_GAP / 2
}

/// Sessions above projects, the sessions list bounded so the projects caption
/// always has somewhere to land.
pub fn sections(r: Rect, sessions: u32) -> Sections {
    let x = r.x + RAIL_PAD;
    let w = r.w.saturating_sub(RAIL_PAD * 2);
    let bottom = r.y + r.h;
    let hh = head_h();
    let s_head = Rect { x, y: r.y + RAIL_GAP, w, h: hh };
    let list_y = s_head.y + hh;
    let room = bottom.saturating_sub(list_y + hh + RAIL_GAP + RAIL_PAD);
    let s_list = Rect { x, y: list_y, w, h: (sessions * row_h()).min(room) };
    let p_head = Rect { x, y: s_list.y + s_list.h + RAIL_GAP, w, h: hh };
    let p_list =
        Rect { x, y: p_head.y + hh, w, h: bottom.saturating_sub(p_head.y + hh + RAIL_PAD) };
    Sections { s_head, s_plus: plus_rect(s_head), s_list, p_head, p_plus: plus_rect(p_head), p_list }
}

pub fn plus_rect(head: Rect) -> Rect {
    let s = lh();
    Rect { x: head.x + head.w.saturating_sub(s), y: head.y + RAIL_GAP, w: s, h: s }
}

pub fn hit(r: Rect, sessions: u32, projects: u32, x: u32, y: u32) -> Option<LeftHit> {
    let s = sections(r, sessions);
    if inside(s.s_plus, x, y) {
        return Some(LeftHit::NewSession);
    }
    if inside(s.p_plus, x, y) {
        return Some(LeftHit::AddProject);
    }
    if let Some(i) = row_at(s.s_list, sessions, x, y) {
        return Some(LeftHit::Session(i));
    }
    row_at(s.p_list, projects, x, y).map(LeftHit::Project)
}
