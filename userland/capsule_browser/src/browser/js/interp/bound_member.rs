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

use alloc::rc::Rc;

use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::style_get::style_get;

// Member reads on a node facet: style properties come from the inline
// style attribute; classList only answers methods, handled at call sites.
pub(super) fn bound_member(ctx: &mut Ctx, kind: &str, id: usize, name: &str) -> Value {
    match kind {
        "style" => Value::Str(Rc::new(style_get(ctx.dom, id, name))),
        _ => Value::Undef,
    }
}
