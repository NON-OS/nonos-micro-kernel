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
use nonos_app_skeleton::clients::vfs::read_file;

const MAX: u32 = 65536;

// The four artifact blobs of a capsule, read from the VFS store.
pub(super) struct Artifacts {
    pub elf: Vec<u8>,
    pub cert: Vec<u8>,
    pub manifest: Vec<u8>,
    pub trailer: Vec<u8>,
}

// Read /capsules/<stem>.{elf,nonos_id_cert.bin,manifest.bin,zk_trailer.bin}
// from the VFS using the caller's pid.
pub(super) fn read_artifacts(pid: u32, stem: &[u8]) -> Result<Artifacts, &'static str> {
    Ok(Artifacts {
        elf: read_one(pid, stem, b".elf")?,
        cert: read_one(pid, stem, b".nonos_id_cert.bin")?,
        manifest: read_one(pid, stem, b".manifest.bin")?,
        trailer: read_one(pid, stem, b".zk_trailer.bin")?,
    })
}

fn read_one(pid: u32, stem: &[u8], ext: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut path = Vec::with_capacity(10 + stem.len() + ext.len());
    path.extend_from_slice(b"/capsules/");
    path.extend_from_slice(stem);
    path.extend_from_slice(ext);
    read_file(pid, &path, MAX)
}
