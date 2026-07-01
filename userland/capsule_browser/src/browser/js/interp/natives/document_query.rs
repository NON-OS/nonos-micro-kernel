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

use alloc::string::ToString;

use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;
use super::find::find_node;

pub fn query(ctx: &mut Ctx, argv: &[Value]) -> Value {
    let sel = argv.first().map(to_str).unwrap_or_default();
    if let Some(id) = sel.strip_prefix('#') {
        let id = id.to_string();
        return find_node(ctx.dom, |n| n.attr("id") == Some(id.as_str()));
    }
    if let Some(cls) = sel.strip_prefix('.') {
        let cls = cls.to_string();
        return find_node(ctx.dom, |n| n.attr("class").is_some_and(|c| c.split_whitespace().any(|x| x == cls)));
    }
    let tag = sel.to_ascii_lowercase();
    find_node(ctx.dom, |n| n.tag == tag)
}
