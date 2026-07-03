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

use super::css_name::css_name;
use super::ctx::Ctx;

const MAX_STYLE_LEN: usize = 2048;

// Write one property into the element's inline style attribute, replacing
// an existing declaration of the same name.
pub(super) fn style_set(ctx: &mut Ctx, id: usize, prop: &str, value: &str) {
    let want = css_name(prop);
    let Some(node) = ctx.dom.nodes.get(id) else {
        return;
    };
    let mut out = String::new();
    for d in node.attr("style").unwrap_or("").split(';') {
        let Some(colon) = d.find(':') else {
            continue;
        };
        if d[..colon].trim().eq_ignore_ascii_case(&want) {
            continue;
        }
        let (k, v) = (d[..colon].trim(), d[colon + 1..].trim());
        if !k.is_empty() && !v.is_empty() && out.len() < MAX_STYLE_LEN {
            out.push_str(k);
            out.push(':');
            out.push_str(v);
            out.push(';');
        }
    }
    let value = value.trim();
    if !value.is_empty() && out.len() + want.len() + value.len() + 2 <= MAX_STYLE_LEN {
        out.push_str(&want);
        out.push(':');
        out.push_str(value);
        out.push(';');
    }
    ctx.dom.set_attr(id, "style", out);
    ctx.dirty = true;
}
