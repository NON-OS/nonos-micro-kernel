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

use crate::browser::js::value::Value;

use super::json_parse::json_parse;

// Parsed body for responses that carry JSON; anything else stays undefined.
pub(in crate::browser::js::interp) fn json_of(body: &str) -> Value {
    let t = body.trim_start();
    if t.starts_with('{') || t.starts_with('[') {
        json_parse(t)
    } else {
        Value::Undef
    }
}
