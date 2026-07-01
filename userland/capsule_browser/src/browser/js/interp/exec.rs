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

use crate::browser::js::ast::Stmt;
use crate::browser::js::env::Env;

use super::ctx::Ctx;
use super::eval_stmt::eval_stmt;
use super::flow::Flow;
use super::hoist::hoist;

pub fn exec(ctx: &mut Ctx, env: &Env, stmts: &[Stmt]) -> Result<Flow, ()> {
    hoist(env, stmts);
    for s in stmts {
        match eval_stmt(ctx, env, s)? {
            Flow::Normal => {}
            other => return Ok(other),
        }
    }
    Ok(Flow::Normal)
}
