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

use super::action::Action;
use crate::ui::layout::{Layout, Rect};
use crate::ui::player::header::{back_button, info_button};
use crate::ui::player::transport::transport;
use crate::ui::screen::Route;

const SCRUB_GRAB: u32 = 9;
const SKIP_SECS: i32 = 10;
const GRAB: u32 = 6;

pub fn from_click(l: &Layout, w: u32, x: i32, y: i32) -> Action {
    if x < 0 || y < 0 {
        return Action::None;
    }
    if back_button().contains(x, y) {
        return Action::Back;
    }
    if info_button(w).contains(x, y) {
        return Action::Goto(Route::Details);
    }
    let t = transport(l, w);
    if t.play.contains(x, y) {
        return Action::TogglePlay;
    }
    if t.prev.contains(x, y) {
        return Action::SeekBy(-SKIP_SECS);
    }
    if t.next.contains(x, y) {
        return Action::SeekBy(SKIP_SECS);
    }
    if grown(t.mute).contains(x, y) {
        return Action::ToggleMute;
    }
    if let Some(level) = along(grown(t.volume), x, y) {
        return Action::SetVolume(level / 10);
    }
    if let Some(permille) = scrub_permille(l, x as u32, y as u32) {
        return Action::SeekToPermille(permille);
    }
    if l.video.contains(x, y) {
        return Action::TogglePlay;
    }
    Action::None
}

fn grown(r: Rect) -> Rect {
    Rect {
        x: r.x.saturating_sub(GRAB),
        y: r.y.saturating_sub(GRAB),
        w: r.w + GRAB * 2,
        h: r.h + GRAB * 2,
    }
}

fn along(r: Rect, x: i32, y: i32) -> Option<u32> {
    if !r.contains(x, y) {
        return None;
    }
    let span = r.w.saturating_sub(1).max(1) as u64;
    Some(((x as u32 - r.x) as u64 * 1000 / span).min(1000) as u32)
}

fn scrub_permille(l: &Layout, x: u32, y: u32) -> Option<u32> {
    let band_y = l.scrub.y.saturating_sub(SCRUB_GRAB);
    let band_h = l.scrub.h + SCRUB_GRAB * 2;
    if x < l.scrub.x || x >= l.scrub.x + l.scrub.w {
        return None;
    }
    if y < band_y || y >= band_y + band_h {
        return None;
    }
    let span = l.scrub.w.saturating_sub(1).max(1) as u64;
    let permille = (x - l.scrub.x) as u64 * 1000 / span;
    Some(permille.min(1000) as u32)
}
