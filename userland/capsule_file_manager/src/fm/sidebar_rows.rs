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

use alloc::{string::String, string::ToString, vec::Vec};

use super::layout::SIDE_FIRST_Y;
use super::paint_sidebar::PLACES;
use super::screen::Screen;
use super::sidebar_model::{Rows, SideHit, SideKind, SideRow, DOWNLOADS, NAV, SEC_GAP};
use super::sidebar_storage::{CARD_FOOT, CARD_H};
use super::state::State;

/// The one layout pass: `paint_sidebar` draws these rows and `side_hit` tests
/// against them, so a click can never land where nothing was drawn. Favorites is
/// variable-length, so every row below it is placed by the running y, not by index.
pub fn side_rows(state: &State) -> Vec<SideRow> {
    let mut b = Rows { rows: Vec::new(), y: SIDE_FIRST_Y };
    for (text, screen) in NAV {
        b.row(text.to_string(), SideHit::Screen(screen));
    }
    b.rule();
    let favorites = state.favorites.list();
    if !favorites.is_empty() {
        b.label("FAVORITES");
        for path in favorites {
            b.row(base_label(path), SideHit::Path(path.to_string()));
        }
        b.y += SEC_GAP;
    }
    b.label("PLACES");
    b.row("Downloads".to_string(), SideHit::Path(DOWNLOADS.to_string()));
    for (text, path) in PLACES {
        b.row(text.to_string(), SideHit::Path(path.to_string()));
    }
    storage_slot(state, &mut b);
    b.rows
}

// The drive card is pinned to the foot of the rail rather than stacked after the
// groups, so it is placed off the window height and withheld entirely when the
// groups above it would overlap it. Its click browses the store root, which is
// the destination PLACES already names, so no new one is invented for it.
fn storage_slot(state: &State, b: &mut Rows) {
    let y = state.win_h.saturating_sub(CARD_H + CARD_FOOT);
    if y < b.y + CARD_FOOT {
        return;
    }
    let hit = SideHit::Path(PLACES[0].1.to_string());
    b.pinned(y, CARD_H, SideKind::Storage, Some(hit));
}

/// Which row `y` lands on; `None` on a section label, a rule, or a gap.
pub fn side_hit(state: &State, y: u32) -> Option<SideHit> {
    side_rows(state)
        .into_iter()
        .find(|r| r.hit.is_some() && y >= r.y && y < r.y + r.h)
        .and_then(|r| r.hit)
}

/// Active when the row is the drawn surface, or Browse is in the path it names.
pub fn row_active(state: &State, hit: &SideHit) -> bool {
    match hit {
        SideHit::Screen(s) => state.screen == *s,
        SideHit::Path(p) => state.screen == Screen::Browse && state.prefix.as_str() == p.as_str(),
    }
}

/// A favorite shows its last path segment; a path with no segment keeps itself.
pub fn base_label(path: &str) -> String {
    match path.trim_end_matches('/').rsplit('/').next() {
        Some(name) if !name.is_empty() => name.to_string(),
        _ => path.to_string(),
    }
}
