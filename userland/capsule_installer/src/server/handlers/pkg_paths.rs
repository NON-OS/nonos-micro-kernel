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

use nonos_app_skeleton::clients::vfs::stat;

// Extensions match the four artifacts `load_by_name` reads back; the order
// zips with the blobs array in `pkg_commit`.
pub(super) const EXTS: [&[u8]; 4] =
    [b".elf", b".manifest.bin", b".nonos_id_cert.bin", b".zk_trailer.bin"];

pub(super) fn artifact_path(name: &[u8], ext: &[u8]) -> Vec<u8> {
    let mut path = Vec::with_capacity(10 + name.len() + ext.len());
    path.extend_from_slice(b"/capsules/");
    path.extend_from_slice(name);
    path.extend_from_slice(ext);
    path
}

// Any single surviving artifact claims the slot. A partial install left by a
// failed or interrupted removal must be cleared with OP_PKG_REMOVE before the
// name is reusable, so a different publisher's package cannot overwrite the
// remains of another's under the same slug.
pub(super) fn installed(pid: u32, name: &[u8]) -> bool {
    EXTS.iter().any(|ext| stat(pid, &artifact_path(name, ext)).is_ok())
}
