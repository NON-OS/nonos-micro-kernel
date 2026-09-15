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

//! The per-region hit-tests behind `hit`: the transport bar and the content
//! column. Both read the painters' own geometry functions.

use super::control::Control;
use super::geometry::Rect;
use super::hit::Action;
use super::screen;
use super::shell as chrome;
use super::state::{UiState, View};
use super::widget::{hero_action_at, permille};

pub fn bar_hit(r: Rect, x: i32, y: i32) -> Option<Action> {
    let b = chrome::bar(r);
    let c = if b.play.contains(x, y) {
        Control::PlayPause
    } else if b.prev.contains(x, y) {
        Control::Prev
    } else if b.next.contains(x, y) {
        Control::Next
    } else if b.speaker.contains(x, y) {
        Control::Mute
    } else if b.shuffle.contains(x, y) {
        Control::Shuffle
    } else if b.repeat.contains(x, y) {
        Control::Repeat
    } else if b.scrub.inset(-10).contains(x, y) {
        Control::Seek(permille(b.scrub, x))
    } else if b.volume.inset(-10).contains(x, y) {
        Control::Volume(permille(b.volume, x))
    } else {
        return None;
    };
    Some(Action::Ctl(c))
}

pub fn content_hit(ui: &UiState, p: Rect, rows: &[usize], n: usize, x: i32, y: i32) -> Option<Action> {
    match ui.view {
        View::Home => match hero_action_at(p, x, y) {
            Some(0) => Some(Action::Select(0)),
            Some(_) => Some(Action::Ctl(Control::Shuffle)),
            None => screen::card_at(p, n, x, y).map(Action::Select),
        },
        View::Browse => screen::card_at(p, n, x, y).map(Action::Select),
        View::Library => {
            if let Some(t) = screen::tab_hit(p, x, y) {
                return Some(Action::LibTab(t));
            }
            screen::lib_row_at(p, ui.scroll, rows.len(), x, y)
                .and_then(|i| rows.get(i).copied())
                .map(Action::Select)
        }
        View::Search if screen::search_play_at(p, rows, x, y).is_some() => {
            screen::search_play_at(p, rows, x, y).map(Action::Select)
        }
        View::Search => screen::search_row_at(p, ui.scroll, rows.len(), x, y)
            .and_then(|i| rows.get(i).copied())
            .map(Action::Select),
        View::Settings => screen::settings_hit(p, x, y).map(|h| match h {
            screen::SettingsHit::Toggle(0) => Action::Ctl(Control::Shuffle),
            screen::SettingsHit::Toggle(1) => Action::Ctl(Control::Repeat),
            screen::SettingsHit::Toggle(_) => Action::Ctl(Control::Mute),
            screen::SettingsHit::Volume(p) => Action::Ctl(Control::Volume(p)),
            screen::SettingsHit::Section(i) => Action::Section(i),
        }),
        View::Downloads => {
            screen::downloads_row_at(p, n.min(screen::downloads_visible(p)), x, y).map(Action::Select)
        }
        View::Radio => screen::radio_tile_at(p, x, y).map(Action::Select),
        View::NowPlaying => None,
    }
}

