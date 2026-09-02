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
use super::rail_band::{hits, Band};
use super::rail_row::{row_at, row_h};
use super::rail_text::{lh, RAIL_GAP, RAIL_PAD};
use crate::layout::Rect;

pub enum LeftHit {
    NewSession,
    AddProject,
    Session(u32),
    Project(u32),
}

pub struct Sections {
    pub s_head: Band,
    pub s_plus: Band,
    pub s_list: Band,
    pub p_head: Band,
    pub p_plus: Band,
    pub p_list: Band,
}

/// Height `rail_text::head` consumes, so the stack below a caption is derived
/// once instead of guessed by each caller.
pub fn head_h() -> u32 {
    RAIL_GAP + lh() + RAIL_GAP / 2
}

/// What the two navigation lists take out of the scrolled column. Every row is
/// laid out: the column scrolls rather than the lists truncating.
pub fn nav_h(sessions: u32, projects: u32) -> u32 {
    (RAIL_GAP + head_h()) * 2 + (sessions + projects) * row_h()
}

pub fn sections(x: u32, w: u32, top: i32, sessions: u32, projects: u32) -> Sections {
    let hh = head_h();
    let s_head = Band { x, y: top + RAIL_GAP as i32, w, h: hh };
    let s_list = Band { x, y: s_head.y + hh as i32, w, h: sessions * row_h() };
    let p_head = Band { x, y: s_list.y + (s_list.h + RAIL_GAP) as i32, w, h: hh };
    let p_list = Band { x, y: p_head.y + hh as i32, w, h: projects * row_h() };
    let (s_plus, p_plus) = (plus_band(s_head), plus_band(p_head));
    Sections { s_head, s_plus, s_list, p_head, p_plus, p_list }
}

/// The one entry point the painter and the hit-test share, so a click resolves
/// against the geometry the last frame actually drew at this offset.
pub fn nav_sections(r: Rect, offset: u32, sessions: u32, projects: u32) -> Sections {
    let w = r.w.saturating_sub(RAIL_PAD * 2);
    sections(RAIL_PAD, w, -(offset as i32), sessions, projects)
}

pub fn plus_band(head: Band) -> Band {
    let s = lh();
    Band { x: head.x + head.w.saturating_sub(s), y: head.y + RAIL_GAP as i32, w: s, h: s }
}

pub fn hit(r: Rect, sessions: u32, projects: u32, off: u32, x: u32, y: u32) -> Option<LeftHit> {
    let s = nav_sections(r, off, sessions, projects);
    let (x, y) = (x.saturating_sub(r.x), y as i32 - r.y as i32);
    if hits(&s.s_plus, x, y) {
        return Some(LeftHit::NewSession);
    }
    if hits(&s.p_plus, x, y) {
        return Some(LeftHit::AddProject);
    }
    if let Some(i) = row_at(&s.s_list, sessions, x, y) {
        return Some(LeftHit::Session(i));
    }
    row_at(&s.p_list, projects, x, y).map(LeftHit::Project)
}
