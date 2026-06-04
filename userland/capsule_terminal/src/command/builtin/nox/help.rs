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

use crate::command::output::Output;

pub fn run(out: &mut Output<'_>) {
    out.writeln(b"nox commands");
    out.writeln(b"  where            current location");
    out.writeln(b"  in <path>        change location");
    out.writeln(b"  ls [path]        list entries");
    out.writeln(b"  read <file>      print a file");
    out.writeln(b"  write <f> <txt>  write a file");
    out.writeln(b"  copy <a> <b>     duplicate a file");
    out.writeln(b"  mk <dir>         make a directory");
    out.writeln(b"  rm <path>        remove a file or empty dir");
    out.writeln(b"  mv <a> <b>       move or rename");
    out.writeln(b"  stat <path>      metadata");
    out.writeln(b"  caps             running capsules");
    out.writeln(b"  svc <name>       resolve a service");
    out.writeln(b"  id               identity");
    out.writeln(b"  sys              version and identity");
    out.writeln(b"  apps             marketplace catalog");
    out.writeln(b"  ping <name>      probe a service");
    out.writeln(b"  display          display info");
    out.writeln(b"  history          command history");
    out.writeln(b"  motd             banner");
    out.writeln(b"  echo <text>      print text");
    out.writeln(b"  clear            clear screen");
    out.writeln(b"  help             this index");
}
