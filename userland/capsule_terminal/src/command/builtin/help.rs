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

//! List the available commands, grouped, so a new user can discover the shell
//! without leaving it.

use crate::command::output::Output;

pub fn run(out: &mut Output<'_>) {
    out.writeln(b"files    ls  cat  cd  pwd  mkdir  touch  rm  rmdir  mv  cp  stat  find  du");
    out.writeln(b"         basename  dirname  pull  push");
    out.writeln(b"text     head  tail  grep  wc  echo  (pipe: sort nl uniq cut)");
    out.writeln(b"shell    |  >  >>  <   alias  unalias  set  unset  env  history  clear  Ctrl-L");
    out.writeln(b"         jobs  fg  bg  exec  run/open  exit");
    out.writeln(b"tabs     Ctrl+Shift+T new   Ctrl+Shift+W close   Ctrl+PgUp/PgDn switch");
    out.writeln(b"zoom     Ctrl+=  bigger   Ctrl+-  smaller");
    out.writeln(b"system   capsules  service  ps  kill  sys  id  whoami  date  uptime  battery");
    out.writeln(b"         version  about  motd  neofetch  display  theme/profile");
    out.writeln(b"net      ping  ifconfig/ip  nslookup/host  curl/http  nym");
    out.writeln(b"apps     apps/market  install  pkg  git");
    out.writeln(b"nox      nox <cmd>   (run 'nox help' for the chain tools)");
}
