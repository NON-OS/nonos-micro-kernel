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

use nonos_app_skeleton::{EventOutcome, KEY_ENTER};

use super::event_browse::on_browse_key;
use super::event_grid::grid_select;
use super::event_head::on_head;
use super::event_side::on_side;
use super::header_hit::head_hit;
use super::info_hit::info_click;
use super::layout::{HEADER_H, SIDEBAR_W};
use super::list_click::list_click;
use super::navigate::navigate;
use super::screen::Screen;
use super::screen_hit::{screen_hit, ScreenHit};
use super::sel_hit::band_click;
use super::state::{Mode, State, ViewKind};

/// Resolve a click against the chrome in the order it is stacked: the sidebar
/// owns its whole column, the header its whole band, and only what is left
/// reaches the active surface. A modal dialog is keyboard-driven, so a click
/// while one is up is swallowed rather than acting on the chrome behind it.
pub fn on_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    if !matches!(state.mode, Mode::Browse) {
        return EventOutcome::Idle;
    }
    if x < SIDEBAR_W {
        return on_side(state, y);
    }
    if y < HEADER_H {
        return match head_hit(state, x, y) {
            Some(hit) => on_head(state, hit),
            None => EventOutcome::Idle,
        };
    }
    if state.screen != Screen::Browse {
        return on_screen(state, x, y);
    }
    if let Some(outcome) = band_click(state, x, y) {
        return outcome;
    }
    if let Some(outcome) = info_click(state, x, y) {
        return outcome;
    }
    match state.view {
        ViewKind::Grid => {
            if !grid_select(state, x, y) {
                return EventOutcome::Idle;
            }
            on_browse_key(state, KEY_ENTER)
        }
        ViewKind::List => list_click(state, x, y),
    }
}

// A stacked surface resolves to a path to open or a tag to filter by. Clicking
// the active tag again clears the filter, which is the only way back to the
// bare chip row.
fn on_screen(state: &mut State, x: u32, y: u32) -> EventOutcome {
    match screen_hit(state, x, y) {
        Some(ScreenHit::Open(path)) => {
            navigate(state, path.as_str());
            EventOutcome::Repaint
        }
        Some(ScreenHit::Go(screen)) => {
            state.screen = screen;
            EventOutcome::Repaint
        }
        Some(ScreenHit::Filter(kind)) => {
            match state.screen {
                Screen::Recents => state.recents_filter = kind,
                _ => state.hit_filter = kind,
            }
            EventOutcome::Repaint
        }
        Some(ScreenHit::Tag(name)) => {
            state.tag_filter =
                if state.tag_filter == name { String::new() } else { name };
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
