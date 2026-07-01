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

use crate::browser::dom::Dom;

use super::collect_scripts::collect_scripts;
use super::env::Env;
use super::interp::{drain, exec, install, Ctx};
use super::lexer::tokenize;
use super::parse::{program, Parser};

const STEP_BUDGET: u64 = 5_000_000;

pub fn run(dom: &mut Dom) -> (bool, String) {
    let scripts = collect_scripts(dom);
    if scripts.is_empty() {
        return (false, String::new());
    }
    let mut ctx = Ctx::new(dom, STEP_BUDGET);
    let env = Env::root();
    install(&env);
    for src in &scripts {
        let prog = program(&mut Parser::new(tokenize(src)));
        let _ = exec(&mut ctx, &env, &prog);
        if ctx.steps >= ctx.budget {
            break;
        }
    }
    drain(&mut ctx, &env);
    (ctx.dirty, ctx.out)
}
