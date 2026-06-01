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

pub fn run(out: &mut Output<'_>, _argv: &[&[u8]]) {
    out.writeln(b"NONOS terminal -- available commands:");
    out.writeln(b"  help                    show this list");
    out.writeln(b"  version                 kernel + capsule version");
    out.writeln(b"  echo <text>             print text");
    out.writeln(b"  clear                   clear scrollback");
    out.writeln(b"  history                 show command history");
    out.writeln(b"  service <name>          lookup IPC service port");
    out.writeln(b"  ping <service>          call OP_HEALTHCHECK on service");
    out.writeln(b"  capsules                list expected capsule services");
    out.writeln(b"  display                 query primary display dims");
    out.writeln(b"  market                  list capsules available from the marketplace");
    out.writeln(b"  motd                    reprint the welcome banner");
    out.writeln(b"  whoami                  show this capsule's identity");
    out.writeln(b"  about                   describe this terminal");
    out.writeln(b"  exit                    leave the terminal (Esc also works)");
    out.writeln(b"");
    out.writeln(b"keyboard:");
    out.writeln(b"  arrows + home/end       move cursor in input line");
    out.writeln(b"  up/down                 walk command history");
    out.writeln(b"  page-up/page-down       scroll output");
    out.writeln(b"  Ctrl-L                  clear scrollback");
    out.writeln(b"  Ctrl-C                  cancel current input line");
    out.writeln(b"  Ctrl-U                  drop the current input line");
    out.writeln(b"  Ctrl-A / Ctrl-E         home / end of input line");
    out.writeln(b"  Ctrl-Shift-C            copy input line to clipboard");
    out.writeln(b"  Ctrl-V                  paste from clipboard");
}
