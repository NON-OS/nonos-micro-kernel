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

use nonos_app_skeleton::{
    EventOutcome, InputEvent, InputKind, KEY_BACKSPACE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_HOME,
    KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP, MOD_SHIFT,
};

use crate::browser::keymap::printable;
use crate::browser::paint::chrome::{self, Btn, TITLEBAR};
use crate::browser::paint::document::VIEW_H;
use crate::browser::paint::home_page::{self, CONTENT_TOP};
use crate::browser::state::{State, View};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.kind {
        InputKind::ButtonDown => on_button(state, event),
        InputKind::Wheel => {
            scroll_by(state, -event.delta_y * 60);
            EventOutcome::Repaint
        }
        InputKind::KeyDown if state.address_focused => on_key(state, event),
        InputKind::KeyDown if matches!(state.view, View::Page) => on_page_key(state, event),
        _ => EventOutcome::Idle,
    }
}

fn scroll_by(state: &mut State, dy: i32) {
    let max = state
        .document
        .as_ref()
        .map(|d| d.content_h.saturating_sub(VIEW_H))
        .unwrap_or(0);
    let next = state.scroll as i32 + dy;
    state.scroll = next.clamp(0, max as i32) as u32;
}

fn on_page_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let page = VIEW_H as i32 - 40;
    match event.code {
        KEY_UP => scroll_by(state, -40),
        KEY_DOWN => scroll_by(state, 40),
        KEY_PAGE_UP => scroll_by(state, -page),
        KEY_PAGE_DOWN | 0x20 => scroll_by(state, page),
        KEY_HOME => state.scroll = 0,
        KEY_END => scroll_by(state, i32::MAX),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

fn on_button(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.y < TITLEBAR as i32 {
        return EventOutcome::Idle;
    }
    if event.y < CONTENT_TOP as i32 {
        return on_toolbar(state, event);
    }
    match state.view {
        View::Home => on_home_click(state, event),
        View::Page => on_page_click(state, event),
    }
}

fn on_toolbar(state: &mut State, event: InputEvent) -> EventOutcome {
    match chrome::toolbar_button_at(event.x, event.y) {
        Some(Btn::Home) => {
            state.view = View::Home;
            state.address.clear();
            EventOutcome::Repaint
        }
        Some(Btn::Reload) => {
            if !state.address.is_empty() {
                state.pending_nav = Some(state.address.clone());
            }
            EventOutcome::Repaint
        }
        Some(Btn::Url) => {
            state.address_focused = true;
            EventOutcome::Repaint
        }
        Some(Btn::Back) => nav_history(state, -1),
        Some(Btn::Forward) => nav_history(state, 1),
        None => {
            state.address_focused = false;
            EventOutcome::Idle
        }
    }
}

fn on_home_click(state: &mut State, event: InputEvent) -> EventOutcome {
    if home_page::search_bar_hit(event.x, event.y) {
        state.address_focused = true;
        return EventOutcome::Repaint;
    }
    match home_page::shortcut_at(event.x, event.y) {
        Some(url) => {
            state.address = url.into();
            state.pending_nav = Some(url.into());
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}

fn on_page_click(state: &mut State, event: InputEvent) -> EventOutcome {
    let dy = event.y - CONTENT_TOP as i32 + state.scroll as i32;
    let hit = state
        .document
        .as_ref()
        .and_then(|d| d.link_at(event.x, dy).map(alloc::string::String::from));
    if let Some(href) = hit {
        state.address = href.clone();
        state.pending_nav = Some(href);
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}

fn nav_history(state: &mut State, delta: i32) -> EventOutcome {
    if state.fetch.is_some() {
        return EventOutcome::Idle;
    }
    let next = state.hist_index + delta;
    if next < 0 || next >= state.history.len() as i32 {
        return EventOutcome::Idle;
    }
    state.hist_index = next;
    state.suppress_history_push = true;
    let url = state.history[next as usize].clone();
    state.address = url.clone();
    state.pending_nav = Some(url);
    EventOutcome::Repaint
}

fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.code {
        KEY_ENTER => {
            state.pending_nav = Some(state.address.clone());
            state.status = alloc::format!("loading {}", state.address);
            state.address_focused = false;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.address.pop();
            EventOutcome::Repaint
        }
        code => match printable(code, event.flags & MOD_SHIFT != 0) {
            Some(b) => {
                state.address.push(b as char);
                EventOutcome::Repaint
            }
            None => EventOutcome::Idle,
        },
    }
}
