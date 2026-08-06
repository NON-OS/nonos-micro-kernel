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

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

const PATH: &[u8] = b"/readme.txt";
const MAX_BYTES: u32 = 64 * 1024;

pub fn read_doc() -> Result<String, &'static str> {
    let owner_pid = mk_getpid();
    if owner_pid == 0 {
        return Err("mdview: cannot resolve own pid");
    }
    if let Ok((size, is_dir)) = vfs::stat(owner_pid, PATH) {
        if is_dir || size > MAX_BYTES as u64 {
            return Err("mdview: /readme.txt is a directory or too large");
        }
    }
    let bytes = vfs::read_file(owner_pid, PATH, MAX_BYTES)?;
    String::from_utf8(bytes).map_err(|_| "mdview: /readme.txt is not valid utf-8")
}
