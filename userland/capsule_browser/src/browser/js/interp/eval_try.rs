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

use crate::browser::js::ast::Stmt;
use crate::browser::js::env::Env;

use super::ctx::Ctx;
use super::exec::exec;
use super::flow::Flow;

type Catch = Option<(Option<String>, Vec<Stmt>)>;

// try/catch/finally. A caught throw is a pending exception in the context; a
// hard engine abort (no pending exception) is not caught. Finally runs in all
// cases and its abrupt completion supersedes the try/catch outcome.
pub fn eval_try(
    ctx: &mut Ctx,
    env: &Env,
    body: &[Stmt],
    catch: &Catch,
    finally: &Option<Vec<Stmt>>,
) -> Result<Flow, ()> {
    let mut result = exec(ctx, &env.child(), body);
    if result.is_err() {
        if let Some(exc) = ctx.exception.take() {
            match catch {
                Some((param, cbody)) => {
                    let cenv = env.child();
                    if let Some(p) = param {
                        cenv.define(p, exc);
                    }
                    result = exec(ctx, &cenv, cbody);
                }
                None => ctx.exception = Some(exc),
            }
        }
    }
    if let Some(fin) = finally {
        match exec(ctx, &env.child(), fin) {
            Ok(Flow::Normal) => {}
            other => return other,
        }
    }
    result
}
