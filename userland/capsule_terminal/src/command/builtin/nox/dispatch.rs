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

use super::{
    apps, caps, clear, copy, display, echo, enter, help, history, id, ls, mk, motd, mv, ping, read,
    rm, stat, svc, sysinfo, unknown, whereis, write,
};
use crate::command::output::Output;
use crate::command::Outcome;
use crate::term::state::State;

pub fn dispatch(state: &mut State, args: &[&[u8]]) -> Outcome {
    if args.is_empty() {
        help::run(&mut Output::new(&mut state.scrollback));
        return Outcome::Repaint;
    }
    let rest = &args[1..];
    match args[0] {
        b"where" => whereis::run(state),
        b"pwd" => whereis::run(state),
        b"in" => enter::run(state, rest),
        b"cd" => enter::run(state, rest),
        b"ls" => ls::run(state, rest),
        b"read" => read::run(state, rest),
        b"cat" => read::run(state, rest),
        b"write" => write::run(state, rest),
        b"copy" => copy::run(state, rest),
        b"mk" => mk::run(state, rest),
        b"mkdir" => mk::run(state, rest),
        b"rm" => rm::run(state, rest),
        b"mv" => mv::run(state, rest),
        b"stat" => stat::run(state, rest),
        b"caps" => caps::run(state),
        b"svc" => svc::run(state, args),
        b"id" => id::run(state),
        b"sys" => sysinfo::run(state),
        b"echo" => echo::run(state, args),
        b"apps" | b"market" => apps::run(state, args),
        b"ping" => ping::run(state, args),
        b"motd" => motd::run(state),
        b"history" => history::run(state, args),
        b"display" => display::run(state, args),
        b"clear" => clear::run(state),
        b"help" => help::run(&mut Output::new(&mut state.scrollback)),
        other => unknown::run(state, other),
    }
    Outcome::Repaint
}
