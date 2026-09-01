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
use nonos_app_skeleton::clients::vfs::read_file;

use super::exec::exec;
use super::outcome::Outcome;
use super::pipeline::{run_filters, run_pipeline};
use super::redirect::{split, split_input, Plan};
use super::write_redirect::write_redirect;
use crate::command::builtin;
use crate::command::parse::Argv;
use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX_INPUT: u32 = 65536;

pub fn run(state: &mut State, argv: &Argv<'_>) -> Outcome {
    if argv.argc == 0 {
        return Outcome::Repaint;
    }
    let args = &argv.argv[..argv.argc];
    if builtin::exit_check::want_exit(args) {
        return Outcome::Exit;
    }
    let (args_in, in_path) = match split_input(args) {
        Ok(v) => v,
        Err(msg) => {
            state.scrollback.push_line(msg);
            return Outcome::Repaint;
        }
    };
    let (cmd, redir) = match split(&args_in) {
        Plan::Plain => (&args_in[..], None),
        Plan::Redirect { cmd_len, append, path } => (&args_in[..cmd_len], Some((append, path))),
        Plan::Error(msg) => {
            state.scrollback.push_line(msg);
            return Outcome::Repaint;
        }
    };
    let piped = cmd.iter().any(|a| *a == b"|");
    if in_path.is_none() && !piped && redir.is_none() {
        return exec(state, cmd);
    }
    let lines = if let Some(p) = in_path {
        let seed = read_input(state, p);
        run_filters(seed, cmd)
    } else if piped {
        run_pipeline(state, cmd)
    } else {
        state.scrollback.begin_capture();
        let _ = exec(state, cmd);
        state.scrollback.end_capture()
    };
    match redir {
        Some((append, path)) => write_redirect(state, &lines, append, path),
        None => {
            for line in &lines {
                state.scrollback.push_line(line);
            }
        }
    }
    Outcome::Repaint
}

fn read_input(state: &mut State, path_arg: &[u8]) -> Vec<Vec<u8>> {
    let path = resolve(state.cwd.as_bytes(), path_arg);
    match read_file(state.owner_pid, &path, MAX_INPUT) {
        Ok(data) => data.split(|&b| b == b'\n').map(<[u8]>::to_vec).collect(),
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            Vec::new()
        }
    }
}
