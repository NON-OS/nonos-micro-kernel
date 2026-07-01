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

use alloc::vec::Vec;

use crate::browser::js::value::Value;

use super::super::ctx::Ctx;
use super::super::to_str::to_str;

pub fn console_log(ctx: &mut Ctx, argv: &[Value]) -> Result<Value, ()> {
    let line = argv.iter().map(to_str).collect::<Vec<_>>().join(" ");
    if ctx.out.len() < 200_000 {
        ctx.out.push_str(&line);
        ctx.out.push('\n');
    }
    Ok(Value::Undef)
}
