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

use alloc::string::{String, ToString};

use crate::browser::js;
use crate::browser::state::State;
use crate::browser::url;

use super::enclosing_form::enclosing_form;
use super::form_fields::form_fields;
use super::relayout::relayout;

// Submit the form enclosing `from`: run its submit listeners, gather the
// fields, then navigate (POST body or GET query per the form's method).
pub(super) fn submit_form(state: &mut State, from: usize) {
    let Some(form) = enclosing_form(state, from) else {
        return;
    };
    state.focus = None;
    if let (Some(dom), Some(world)) = (state.page_dom.as_mut(), state.world.as_mut()) {
        let (_, dirty) = js::dispatch_event(dom, world, form, "submit");
        if dirty {
            relayout(state);
        }
    }
    let Some(dom) = state.page_dom.as_ref() else {
        return;
    };
    let Some(node) = dom.nodes.get(form) else {
        return;
    };
    let body = form_fields(dom, form);
    let action = node.attr("action").unwrap_or("").to_string();
    let post = node.attr("method").is_some_and(|m| m.eq_ignore_ascii_case("post"));
    let target = match (state.base.as_ref(), action.is_empty()) {
        (Some(base), false) => url::join(base, &action),
        (Some(_), true) => state.address.clone(),
        (None, _) => action,
    };
    if target.is_empty() {
        return;
    }
    if post {
        state.pending_post = Some(body);
        state.pending_nav = Some(target);
    } else {
        let sep = if target.contains('?') { '&' } else { '?' };
        let mut t = target;
        if !body.is_empty() {
            t.push(sep);
            t.push_str(&body);
        }
        state.pending_nav = Some(t);
    }
    state.status = String::from("submitting form");
}
