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

use nonos_libc::{mk_capsule_verify, CapsuleVerifyRequest, CapsuleVerifySummary};
use nonos_pack_core::Sections;

use crate::protocol::EINVAL;

pub(super) const EACCES: i32 = -13;

pub(super) struct Verified<'a> {
    pub digest: [u8; 32],
    pub summary: CapsuleVerifySummary,
    pub sections: Sections<'a>,
}

// The package parse and trailer check are structural only; the authorization
// gate is the kernel verify syscall, which runs the same trust chain a spawn
// would. The digest covers the whole file so a later commit can prove the
// bytes the user consented to are the bytes being installed.
pub(super) fn verify_package(bytes: &[u8]) -> Result<Verified<'_>, i32> {
    let (s, trailer_off) = nonos_pack_core::parse(bytes).map_err(|_| EINVAL)?;
    nonos_pack_core::check_trailer(bytes, trailer_off).map_err(|_| EINVAL)?;
    let req = CapsuleVerifyRequest {
        elf_ptr: s.elf.as_ptr() as u64,
        cert_ptr: s.id_cert.as_ptr() as u64,
        manifest_ptr: s.manifest.as_ptr() as u64,
        trailer_ptr: s.zk_trailer.as_ptr() as u64,
        elf_len: s.elf.len() as u32,
        cert_len: s.id_cert.len() as u32,
        manifest_len: s.manifest.len() as u32,
        trailer_len: s.zk_trailer.len() as u32,
    };
    let mut out = CapsuleVerifySummary::zeroed();
    if mk_capsule_verify(&req, &mut out) != 0 {
        return Err(EACCES);
    }
    Ok(Verified { digest: *blake3::hash(bytes).as_bytes(), summary: out, sections: s })
}

pub(super) fn verified_name(s: &CapsuleVerifySummary) -> &[u8] {
    &s.name[..(s.name_len as usize).min(s.name.len())]
}

pub(super) fn verified_ns(s: &CapsuleVerifySummary) -> &[u8] {
    &s.namespace[..(s.ns_len as usize).min(s.namespace.len())]
}
