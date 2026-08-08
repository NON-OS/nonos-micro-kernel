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

mod enclosing_form;
mod field_at;
mod field_key;
mod form_fields;
mod js_click;
mod js_tick;
mod nav_history;
mod on_button;
mod on_event;
mod on_home_click;
mod on_key;
mod on_page_click;
mod on_page_key;
mod on_toolbar;
mod relayout;
mod script_nav;
mod scroll_by;
mod submit_form;

pub use js_tick::js_tick;
pub use on_event::on_event;
pub use relayout::relayout;
