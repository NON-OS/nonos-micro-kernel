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

use crate::browser::fetch::{constants, render_error, retryable_error, security_error};
use crate::browser::state::{State, View};

pub(super) fn fail(state: &mut State, msg: &str) {
    if retryable_error::retryable_error(msg)
        && !security_error::security_error(msg)
        && state.retries < constants::MAX_RETRIES
        && !state.address.is_empty()
    {
        state.retries += 1;
        state.status = alloc::format!("retry {} - {}", state.retries, msg);
        state.pending_nav = Some(state.address.clone());
    } else {
        state.retries = 0;
        state.status = String::from(msg);
        state.document = Some(render_error::render_error(msg));
        state.box_doc = None;
        state.page_dom = None;
        state.world = None;
        state.view = View::Page;
    }
}
