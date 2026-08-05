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

use super::{pack, unpack, verify};

pub const USAGE: &str = "usage: nonos-pack <command> [args]
  pack   --out <app.nonos> --manifest <M> --elf <E> --id-cert <C> --trailer <T>
         --seed ed25519=<S1> --seed mldsa65=<S2>
  unpack --in <app.nonos> --out-dir <DIR>
  verify --in <app.nonos>";

pub fn dispatch(argv: &[String]) -> Result<(), String> {
    if argv.len() < 2 {
        return Err(USAGE.to_string());
    }
    let rest = &argv[2..];
    match argv[1].as_str() {
        "pack" => pack::run(rest),
        "unpack" => unpack::run(rest),
        "verify" => verify::run(rest),
        "help" | "-h" | "--help" => {
            println!("{}", USAGE);
            Ok(())
        }
        other => Err(format!("unknown subcommand `{}`\n\n{}", other, USAGE)),
    }
}
