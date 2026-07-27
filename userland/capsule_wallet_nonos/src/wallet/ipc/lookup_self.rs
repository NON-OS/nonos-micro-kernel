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

use nonos_libc::mk_getpid;

// The wallet's own pid, as the kernel attributes it to this capsule's IPC. It
// must come from getpid, not a service-name lookup: under the on-demand window
// model the interactive wallet is an instance (app.nonos_wallet.N) with its own
// pid, while a name lookup resolves to the boot instance. The keyring checks
// this value against the IPC sender pid, so a name-resolved pid fails every
// ownership check with EACCES.
pub fn lookup_self_pid() -> Result<u32, i32> {
    match mk_getpid() {
        0 => Err(-11),
        pid => Ok(pid),
    }
}
