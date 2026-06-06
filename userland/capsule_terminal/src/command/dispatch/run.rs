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
use crate::command::parse::Argv;
use crate::term::state::State;

pub fn run(state: &mut State, argv: &Argv<'_>) -> Outcome {
    const HELP: &[u8] = b"help";
    if argv.argc == 0 {
        return Outcome::Repaint;
    }
    let args = &argv.argv[..argv.argc];
    if builtin::exit_check::want_exit(args) {
        return Outcome::Exit;
    }
    match args[0] {
        b"nox" => return builtin::nox::dispatch(state, &args[1..]),
        b"help" => return builtin::nox::dispatch(state, &[HELP]),
        b"about" => builtin::about::run(&mut Output::new(&mut state.scrollback), args),
        b"version" => builtin::version::run(&mut Output::new(&mut state.scrollback), args),
        b"whoami" => builtin::whoami::run(&mut Output::new(&mut state.scrollback), args),
        b"capsules" | b"caps" => builtin::capsules::run(&mut Output::new(&mut state.scrollback), args),
        b"clear" => builtin::clear::run(&mut state.scrollback, args),
        b"display" => builtin::display::run(&mut Output::new(&mut state.scrollback), args),
        b"echo" => builtin::echo::run(&mut Output::new(&mut state.scrollback), args),
        b"history" => builtin::history_cmd::run(
            &mut Output::new(&mut state.scrollback),
            &mut state.history,
            args,
        ),
        b"market" => builtin::market::run(&mut Output::new(&mut state.scrollback), args),
        b"motd" => builtin::motd::run(&mut state.scrollback, args),
        b"ping" => builtin::ping::run(&mut Output::new(&mut state.scrollback), args),
        b"service" | b"svc" => builtin::service::run(&mut Output::new(&mut state.scrollback), args),
        _ => return builtin::nox::dispatch(state, args),
    }
    Outcome::Repaint
}
