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

use super::super::ctx::Ctx;
use super::super::obj::obj;
use super::super::to_str::to_str;

const MAX_REQUESTS: usize = 16;

// fetch(url) or fetch(url, cb): queues a request slot and hands back a
// handle whose then() attaches the callback. Requests with no callback by
// the end of the run are dropped.
pub(super) fn js_fetch(ctx: &mut Ctx, argv: &[Value]) -> Value {
    let url = argv.first().map(to_str).unwrap_or_default();
    if url.is_empty() || url.len() > 2048 || ctx.net.len() >= MAX_REQUESTS {
        return Value::Undef;
    }
    let cb = match argv.get(1) {
        Some(f @ Value::Func(_)) => Some(f.clone()),
        _ => None,
    };
    let id = ctx.net.len();
    ctx.net.push((url, cb));
    obj(&[("__net", Value::Num(id as f64))])
}
