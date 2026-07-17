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

mod builtins;
mod console;
mod dispatch;
mod document_create;
mod document_get;
mod document_query;
mod document_query_all;
mod find;
mod floor;
mod js_fetch;
mod json_of;
mod json_parse;
mod json_stringify;
mod json_value;
mod math;
mod object_static;
mod promise_static;
mod timer_ms;

pub use dispatch::dispatch;
pub(super) use json_of::json_of;
