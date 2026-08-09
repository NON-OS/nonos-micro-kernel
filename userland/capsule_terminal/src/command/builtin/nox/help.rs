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
    out.writeln(b"nox commands  (familiar aliases in parentheses)");
    out.writeln(b"  where (pwd)      current location");
    out.writeln(b"  in (cd) <path>   change location");
    out.writeln(b"  ls (dir) [path]  list entries");
    out.writeln(b"  read (cat) <f>   print a file");
    out.writeln(b"  write <f> <txt>  write a file (quote text with spaces)");
    out.writeln(b"  copy (cp) <a> <b> duplicate a file");
    out.writeln(b"  mk (mkdir) <dir> make a directory");
    out.writeln(b"  rm (del) <path>  remove a file or empty dir");
    out.writeln(b"  mv (move) <a> <b> move or rename");
    out.writeln(b"  stat <path>      metadata");
    out.writeln(b"  caps             running capsules");
    out.writeln(b"  svc <name>       resolve a service");
    out.writeln(b"  id               identity");
    out.writeln(b"  sys              version and identity");
    out.writeln(b"  apps             marketplace catalog");
    out.writeln(b"  run (open) <app> launch an app: files editor settings calc about procs");
    out.writeln(b"  install <name>   verify and load a capsule from /capsules/<name>.*");
    out.writeln(b"  pkg <cmd> [arg]  install <path> [--yes] / remove <name> / status");
    out.writeln(b"  set [name val]   list or define a variable (use as $name)");
    out.writeln(b"  unset <name>     remove a variable");
    out.writeln(b"  alias [n exp]    list or define a command alias");
    out.writeln(b"  unalias <name>   remove an alias");
    out.writeln(b"  ping <host>      ICMP echo a host");
    out.writeln(b"  http (curl) <url> fetch a URL (http example.com | http 1.1.1.1)");
    out.writeln(b"  nslookup <host>  resolve a host via DNS");
    out.writeln(b"  ifconfig (ip)    network interface and lease");
    out.writeln(b"  kill <pid> [sig] terminate a capsule (caps lists pids)");
    out.writeln(b"  uptime           time since boot");
    out.writeln(b"  battery (bat)    charge percentage");
    out.writeln(b"  sd / tokio-smoke bundled crates.io tools, run by name");
    out.writeln(b"  display          display info");
    out.writeln(b"  history          command history");
    out.writeln(b"  motd             banner");
    out.writeln(b"  echo <text>      print text");
    out.writeln(b"  clear            clear screen");
    out.writeln(b"  help             this index");
    out.writeln(b"");
    out.writeln(b"  any command > file    write output to a file");
    out.writeln(b"  any command >> file   append output to a file");
    out.writeln(b"  cmd | grep [-i][-v] p keep (or with -v drop) matching lines");
    out.writeln(b"  cmd | sort            sort lines");
    out.writeln(b"  cmd | uniq            drop repeated adjacent lines");
    out.writeln(b"  cmd | nl              number lines");
    out.writeln(b"  cmd | wc              count lines");
    out.writeln(b"  cmd | head [n]        first n lines (default 10)");
    out.writeln(b"  cmd | tail [n]        last n lines (default 10)");
    out.writeln(b"  a ; b                 run a then b");
    out.writeln(b"  a && b                run b only if a succeeds");
    out.writeln(b"  a || b                run b only if a fails");
    out.writeln(b"");
    out.writeln(b"  Tab completes commands and paths.");
}
