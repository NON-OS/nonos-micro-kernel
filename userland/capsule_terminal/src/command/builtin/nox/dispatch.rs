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
    alias, apps, caps, clear, copy, display, echo, enter, help, history, id, install, ls, mk, motd,
    mv, ping, read, rm, run, set, stat, svc, sysinfo, unalias, unknown, unset, whereis, write,
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
    // Fallible commands return their success; infallible commands always
    // succeed; an unknown command is a failure. The result drives `&&`
    // and `||` sequencing through `state.last_status`.
    let ok = match args[0] {
        b"where" | b"pwd" => {
            whereis::run(state);
            true
        }
        b"in" | b"cd" => enter::run(state, rest),
        b"ls" | b"dir" => {
            ls::run(state, rest);
            true
        }
        b"read" | b"cat" => read::run(state, rest),
        b"write" => write::run(state, rest),
        b"copy" | b"cp" => copy::run(state, rest),
        b"mk" | b"mkdir" => mk::run(state, rest),
        b"rm" | b"del" => rm::run(state, rest),
        b"mv" | b"move" => mv::run(state, rest),
        b"stat" => stat::run(state, rest),
        b"caps" => {
            caps::run(state);
            true
        }
        b"svc" => {
            svc::run(state, args);
            true
        }
        b"id" => {
            id::run(state);
            true
        }
        b"sys" => {
            sysinfo::run(state);
            true
        }
        b"echo" => {
            echo::run(state, args);
            true
        }
        b"apps" | b"market" => {
            apps::run(state, args);
            true
        }
        b"run" | b"open" => run::run(state, rest),
        b"install" => install::run(state, rest),
        b"set" => {
            set::run(state, rest);
            true
        }
        b"unset" => {
            unset::run(state, rest);
            true
        }
        b"alias" => {
            alias::run(state, rest);
            true
        }
        b"unalias" => {
            unalias::run(state, rest);
            true
        }
        b"ping" => {
            ping::run(state, args);
            true
        }
        b"motd" => {
            motd::run(state);
            true
        }
        b"history" => {
            history::run(state, args);
            true
        }
        b"display" => {
            display::run(state, args);
            true
        }
        b"clear" => {
            clear::run(state);
            true
        }
        b"help" => {
            help::run(&mut Output::new(&mut state.scrollback));
            true
        }
        other => {
            unknown::run(state, other);
            false
        }
    };
    state.last_status = ok;
    Outcome::Repaint
}
