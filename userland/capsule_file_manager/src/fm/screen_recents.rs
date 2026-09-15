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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;

use super::file_color::color;
use super::file_kind::kind_of_name;
use super::layout::{CONTENT_X, FOOTER_H, HEADER_H, PAD_X, SECTION_GAP};
use super::recents_chip_paint::paint_chips;
use super::recents_chips::{chips_bottom, kind_chips, KindChip};
use super::recents_tally::{kind_counts, shown};
use super::recents_group::{group, parent_of, rel_time};
use super::screen_list::Line;
use super::screen_row::{screen_row, section_label, LABEL_ADV, LIST_ROW_H};
use super::sidebar_rows::base_label;
use super::state::State;

/// The journal's own millisecond base, or zero when the wall clock is
/// unavailable -- a case `group` reports rather than mislabels.
pub fn now_ms() -> u64 {
    mk_time_millis().max(0) as u64
}

/// The filetype chip row, always counted over the *unfiltered* journal so the
/// axis never collapses to the one chip already selected.
pub fn recents_chip_row(state: &State) -> Vec<KindChip> {
    let rows = shown(&state.recents, None);
    kind_chips(&kind_counts(rows.iter().map(|(_, path)| *path)), state.win_w)
}

/// The chip row and `group` are the single geometry source: the row fixes where
/// the list starts, the grouping decides which sections exist, and both the
/// headings and the rows land in this one line list -- so the hit-test steps the
/// same rhythm the painter drew and a click below a heading cannot slip a row.
pub fn recents_lines(state: &State, now: u64) -> Vec<Line> {
    let bottom = state.win_h.saturating_sub(FOOTER_H);
    let mut out = Vec::new();
    let mut y = chips_bottom(&recents_chip_row(state));
    for (label, rows) in group(now, &shown(&state.recents, state.recents_filter)) {
        if y + LIST_ROW_H > bottom {
            break;
        }
        out.push(Line::head(y, LABEL_ADV, label));
        y += LABEL_ADV;
        for (ms, path) in rows {
            if y + LIST_ROW_H > bottom {
                break;
            }
            out.push(Line::row(y, LIST_ROW_H, path, age(now, ms), path.ends_with('/')));
            y += LIST_ROW_H;
        }
        y += SECTION_GAP;
    }
    out
}

// With no clock there is no honest age to state, so the meta column stays empty
// rather than reporting every entry as "just now".
fn age(now: u64, ms: u64) -> String {
    match now {
        0 => String::new(),
        _ => rel_time(now, ms),
    }
}

pub fn paint_recents(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    let chips = recents_chip_row(state);
    if chips.len() < 2 {
        super::screen_row::empty_state(
            fb,
            x,
            HEADER_H + 40,
            w,
            "No recent files",
            "Opening a file records it in the store journal.",
        );
        return;
    }
    section_label(fb, x, HEADER_H + 16, "RECENTS");
    paint_chips(fb, &chips, state.recents_filter);
    for line in &recents_lines(state, now_ms()) {
        let Some(path) = paint_head(fb, x, line) else { continue };
        let title = base_label(path);
        let row = (title.as_str(), parent_of(path), line.meta.as_str());
        screen_row(fb, x, line.y, w, row, line.dir, color(kind_of_name(path)));
    }
}

// Draws a heading line and reports nothing; a row line reports its path so the
// caller draws it instead.
fn paint_head<'a>(fb: &mut PaintBuffer, x: u32, line: &'a Line) -> Option<&'a str> {
    match line.head {
        Some(text) => {
            section_label(fb, x, line.y, text);
            None
        }
        None => Some(line.path.as_str()),
    }
}
