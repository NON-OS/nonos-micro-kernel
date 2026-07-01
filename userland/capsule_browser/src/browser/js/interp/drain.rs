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

use crate::browser::js::env::Env;

use super::apply::apply;
use super::ctx::Ctx;

pub fn drain(ctx: &mut Ctx, env: &Env) {
    let timers = core::mem::take(&mut ctx.timers);
    for cb in timers.into_iter().take(256) {
        if ctx.steps >= ctx.budget {
            break;
        }
        let _ = apply(ctx, env, cb, Vec::new());
    }
}
