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

use super::outcome::Outcome;
use crate::command::builtin;
use crate::command::output::Output;
use crate::term::state::State;

// Run a single resolved command (already stripped of any redirect). The
// caller handles exit detection and output redirection.
pub(super) fn exec(state: &mut State, args: &[&[u8]]) -> Outcome {
    if args.is_empty() {
        return Outcome::Repaint;
    }
    match args[0] {
        b"nox" => return builtin::nox::dispatch(state, &args[1..]),
        b"about" => builtin::about::run(&mut Output::new(&mut state.scrollback), args),
        b"version" => builtin::version::run(&mut Output::new(&mut state.scrollback), args),
        b"whoami" => builtin::whoami::run(&mut Output::new(&mut state.scrollback), args),
        b"capsules" | b"caps" => {
            builtin::capsules::run(&mut Output::new(&mut state.scrollback), args)
        }
        b"clear" => builtin::clear::run(&mut state.scrollback, args),
        b"display" => builtin::display::run(&mut Output::new(&mut state.scrollback), args),
        b"echo" => builtin::echo::run(&mut Output::new(&mut state.scrollback), args),
        b"history" => {
            builtin::history_cmd::run(&mut Output::new(&mut state.scrollback), &state.history, args)
        }
        b"jobs" => {
            builtin::jobs::run_jobs(&mut Output::new(&mut state.scrollback), &state.jobs, args)
        }
        b"fg" => builtin::jobs::run_fg(state, args),
        b"bg" => builtin::jobs::run_bg(&mut Output::new(&mut state.scrollback), &state.jobs, args),
        b"market" => builtin::market::run(&mut Output::new(&mut state.scrollback), args),
        b"motd" => builtin::motd::run(&mut state.scrollback, args),
        b"neofetch" => builtin::neofetch::run(state),
        b"ping" => builtin::ping::run(&mut Output::new(&mut state.scrollback), args),
        b"service" | b"svc" => builtin::service::run(&mut Output::new(&mut state.scrollback), args),
        b"pwd" => builtin::fs::pwd(state),
        b"cd" => builtin::fs::cd(state, args),
        b"ls" | b"dir" => builtin::fs::ls(state, args),
        b"cat" => builtin::fs::cat(state, args),
        b"mkdir" => builtin::fs::mkdir(state, args),
        b"touch" => builtin::fs::touch(state, args),
        b"rm" | b"del" => builtin::fs::rm(state, args),
        b"mv" => builtin::fs::mv(state, args),
        b"cp" => builtin::fs::cp(state, args),
        b"stat" => builtin::fs::stat(state, args),
        b"rmdir" => builtin::fs::rmdir(state, args),
        b"find" => builtin::fs::find(state, args),
        b"head" => builtin::fs::head(state, args),
        b"tail" => builtin::fs::tail(state, args),
        b"grep" => builtin::fs::grep(state, args),
        b"wc" => builtin::fs::wc(state, args),
        b"help" | b"commands" => builtin::help::run(&mut Output::new(&mut state.scrollback)),
        b"theme" | b"profile" => builtin::theme::run(state, args),
        _ => return builtin::nox::dispatch(state, args),
    }
    Outcome::Repaint
}
