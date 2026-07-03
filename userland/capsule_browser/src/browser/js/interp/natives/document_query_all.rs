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
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::browser::css;
use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;

const QUERY_CAP: usize = 256;

// document.querySelectorAll through the real selector matcher.
pub fn query_all(ctx: &mut Ctx, argv: &[Value]) -> Value {
    let sel = argv.first().map(to_str).unwrap_or_default();
    let hits: Vec<Value> =
        css::select(ctx.dom, &sel, QUERY_CAP).into_iter().map(Value::Node).collect();
    Value::Array(Rc::new(RefCell::new(hits)))
}
