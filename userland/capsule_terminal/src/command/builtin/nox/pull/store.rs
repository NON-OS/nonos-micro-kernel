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

pub fn write(pid: u32, path: &[u8], data: &[u8]) -> Result<(), &'static str> {
    vfs::write_file(pid, path, data)
}

pub fn mkdir(pid: u32, path: &[u8]) {
    let _ = vfs::mkdir(pid, path);
}

pub fn exists(pid: u32, path: &[u8]) -> bool {
    vfs::stat(pid, path).is_ok()
}
