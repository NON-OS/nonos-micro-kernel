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
use alloc::vec::Vec;

use crate::browser::js::value::Value;

use super::ctx::Ctx;
use super::to_str::to_str;

const MAX_CLASSES: usize = 64;

// classList add/remove/toggle/contains against the class attribute.
pub(super) fn classlist_method(ctx: &mut Ctx, id: usize, method: &str, argv: &[Value]) -> Value {
    let name = argv.first().map(to_str).unwrap_or_default();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return Value::Undef;
    }
    let Some(node) = ctx.dom.nodes.get(id) else {
        return Value::Undef;
    };
    let mut classes: Vec<String> =
        node.attr("class").unwrap_or("").split_whitespace().map(String::from).collect();
    let has = classes.iter().any(|c| c == &name);
    let changed = match method {
        "add" => {
            if !has && classes.len() < MAX_CLASSES {
                classes.push(name);
                true
            } else {
                false
            }
        }
        "remove" => {
            classes.retain(|c| c != &name);
            has
        }
        "toggle" => {
            if has {
                classes.retain(|c| c != &name);
            } else if classes.len() < MAX_CLASSES {
                classes.push(name);
            }
            true
        }
        "contains" => return Value::Bool(has),
        _ => false,
    };
    if changed {
        ctx.dom.set_attr(id, "class", classes.join(" "));
        ctx.dirty = true;
    }
    Value::Undef
}
