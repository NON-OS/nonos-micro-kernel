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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{search, SEARCH_CONTENT, SEARCH_NAMES};
use nonos_app_skeleton::PaintBuffer;

use super::entries::RESERVED_PREFIX;
use super::file_color::color;
use super::file_kind::kind_of_name;
use super::filetype::Kind;
use super::layout::{CONTENT_X, FOOTER_H, HEADER_H, PAD_X, SECTION_GAP};
use super::recents_chip_paint::paint_chips;
use super::recents_chips::{chips_bottom, kind_chips, KindChip};
use super::recents_filter::keeps_kind;
use super::recents_tally::kind_tally;
use super::recents_group::parent_of;
use super::screen_list::Line;
use super::screen_row::{empty_state, screen_row, section_label, LABEL_ADV, LIST_ROW_H};
use super::search_hl::highlight;
use super::sidebar_rows::base_label;
use super::state::State;
use super::theme::INK3;

const MAX_HITS: u32 = 256;

/// Runs the query against the store and keeps the hits the manager is willing
/// to show; the reserved sidecar namespace is filtered here so no caller has to
/// remember to. A fresh hit set retires the old chip, whose count no longer
/// describes anything.
pub fn run_search(state: &mut State) {
    state.hits.clear();
    state.hit_filter = None;
    if state.query.is_empty() {
        return;
    }
    let flags = SEARCH_NAMES | SEARCH_CONTENT;
    if let Ok(hits) = search(state.owner_pid, state.query.as_bytes(), flags, MAX_HITS) {
        state.hits =
            hits.into_iter().filter(|(_, _, path)| !path.starts_with(RESERVED_PREFIX)).collect();
    }
}

/// The filetype chip row for the hit set, counted over the *unfiltered* hits so
/// every count is a real number of results and the axis never collapses.
pub fn search_chip_row(state: &State) -> Vec<KindChip> {
    kind_chips(&kind_tally(state.hits.iter().map(hit_kind)), state.win_w)
}

// The store reports kind 2 for a directory hit whether or not the path carries a
// trailing separator, so the chip axis reads that rather than guessing from the
// name -- otherwise a folder would be counted under its extension.
fn hit_kind(hit: &(u32, u32, String)) -> Kind {
    match hit.0 == 2 || hit.2.ends_with('/') {
        true => Kind::Dir,
        false => kind_of_name(hit.2.as_str()),
    }
}

pub fn paint_search(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    if state.query.is_empty() {
        let _ =
            fb.text_ttf(x as i32, (HEADER_H + 24) as i32, "Type to search the store.", INK3, 18.0);
        return;
    }
    if state.hits.is_empty() {
        empty_state(fb, x, HEADER_H + 40, w, "No matches", "Nothing in the store matches that.");
        return;
    }
    section_label(fb, x, HEADER_H + 16, "MATCHES");
    paint_chips(fb, &search_chip_row(state), state.hit_filter);
    for line in &search_lines(state) {
        match line.head {
            Some(text) => {
                section_label(fb, x, line.y, text);
            }
            None => paint_hit(state, fb, x, w, line),
        }
    }
}

// The band goes down first and `screen_row` draws its glyphs over it, so the
// highlight sits behind the text rather than replacing the pixels under it.
fn paint_hit(state: &State, fb: &mut PaintBuffer, x: u32, w: u32, line: &Line) {
    let path = line.path.as_str();
    highlight(fb, x, line.y, path, state.query.as_str());
    let title = base_label(path);
    let row = (title.as_str(), parent_of(path), line.meta.as_str());
    let tint = if line.dir { color(Kind::Dir) } else { color(kind_of_name(path)) };
    screen_row(fb, x, line.y, w, row, line.dir, tint);
}

/// One split of `state.hits` into the screen's line list: name hits (kinds 0
/// and 2) above, content hits (kind 1) below with their 1-based line number in
/// the meta column. The chip row fixes the first y and the headings occupy their
/// own slots, so `screen_hit` walks the exact rhythm drawn here.
pub fn search_lines(state: &State) -> Vec<Line> {
    let bottom = state.win_h.saturating_sub(FOOTER_H);
    let mut out = Vec::new();
    let mut y = chips_bottom(&search_chip_row(state));
    for (label, content) in [("TOP RESULTS", false), ("OTHER RESULTS", true)] {
        let mut headed = false;
        for (kind, line, path) in state.hits.iter().filter(|h| section_of(h, content, state)) {
            if y + LIST_ROW_H > bottom {
                break;
            }
            if !headed {
                out.push(Line::head(y, LABEL_ADV, label));
                y += LABEL_ADV;
                headed = true;
            }
            let meta = if content { alloc::format!("line {line}") } else { String::new() };
            let dir = *kind == 2 || path.ends_with('/');
            out.push(Line::row(y, LIST_ROW_H, path, meta, dir));
            y += LIST_ROW_H;
        }
        if headed {
            y += SECTION_GAP;
        }
    }
    out
}

// A hit belongs to the content section when the store matched inside the file,
// and survives at all only if the active filetype chip keeps it.
fn section_of(hit: &(u32, u32, String), content: bool, state: &State) -> bool {
    (hit.0 == 1) == content && keeps_kind(state.hit_filter, hit_kind(hit))
}
