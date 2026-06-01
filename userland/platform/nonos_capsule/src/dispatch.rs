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

use crate::cmd;

pub fn run(args: &[String]) -> Result<(), String> {
    let command = args.first().map(|s| s.as_str()).unwrap_or("help");
    let rest = if args.is_empty() { &[][..] } else { &args[1..] };
    match command {
        "new" => cmd::new::run(rest),
        "build" => cmd::build::run(rest),
        "manifest" => cmd::manifest::run(rest),
        "sign" => cmd::sign::run(rest),
        "install" => cmd::install::run(rest),
        "run" => cmd::run::run(rest),
        "inspect" => cmd::inspect::run(rest),
        "remove" => cmd::remove::run(rest),
        "help" | "-h" | "--help" => cmd::help::run(),
        other => Err(format!("unknown command `{other}` (try `help`)")),
    }
}
