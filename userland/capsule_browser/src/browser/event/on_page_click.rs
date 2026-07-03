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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::browser::paint::home_page::CONTENT_TOP;
use crate::browser::state::State;
use crate::browser::url;

pub fn on_page_click(state: &mut State, event: InputEvent) -> EventOutcome {
    let dy = event.y - CONTENT_TOP as i32 + state.scroll as i32;
    // Script listeners see the click first; a handled click stops here.
    // Otherwise form fields take focus and submit controls submit.
    state.focus = None;
    if let Some(node) = state.box_doc.as_ref().and_then(|b| b.hit_node(event.x, dy)) {
        if super::js_click::js_click(state, node) {
            return EventOutcome::Repaint;
        }
        if let Some(dom) = state.page_dom.as_ref() {
            match super::field_at::field_at(dom, node) {
                super::field_at::Field::Edit(id) => {
                    state.focus = Some(id);
                    return EventOutcome::Repaint;
                }
                super::field_at::Field::Submit(id) => {
                    super::submit_form::submit_form(state, id);
                    return EventOutcome::Repaint;
                }
                super::field_at::Field::None => {}
            }
        }
    }
    let hit = match state.box_doc.as_ref() {
        Some(b) => b.link_at(event.x, dy).map(alloc::string::String::from),
        None => state
            .document
            .as_ref()
            .and_then(|d| d.link_at(event.x, dy).map(alloc::string::String::from)),
    };
    if let Some(href) = hit {
        let next = match state.base.as_ref() {
            Some(base) => url::join(base, &href),
            None => href,
        };
        state.address = next.clone();
        state.pending_nav = Some(next);
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
