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

use super::ctx::Ctx;
use super::graft_html::graft_html;
use super::set_text_content::set_text_content;
use super::to_str::to_str;

pub fn set_node_prop(ctx: &mut Ctx, id: usize, name: &str, v: &Value) {
    match name {
        "textContent" | "innerText" => {
            set_text_content(ctx.dom, id, to_str(v));
            ctx.dirty = true;
        }
        "innerHTML" => graft_html(ctx, id, &to_str(v)),
        "id" => {
            ctx.dom.set_attr(id, "id", to_str(v));
            ctx.dirty = true;
        }
        "className" => {
            ctx.dom.set_attr(id, "class", to_str(v));
            ctx.dirty = true;
        }
        "value" => {
            ctx.dom.set_attr(id, "value", to_str(v));
            ctx.dirty = true;
        }
        _ => {}
    }
}
