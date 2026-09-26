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

//! A program with a publisher behind it, checked exactly as a capsule is.

use nonos_libc::{mk_capsule_verify, CapsuleVerifyRequest, CapsuleVerifySummary};

pub fn verify(
    image: &[u8],
    cert: &[u8],
    manifest: &[u8],
    trailer: &[u8],
) -> Result<CapsuleVerifySummary, &'static str> {
    let req = CapsuleVerifyRequest {
        elf_ptr: image.as_ptr() as u64,
        cert_ptr: cert.as_ptr() as u64,
        manifest_ptr: manifest.as_ptr() as u64,
        trailer_ptr: trailer.as_ptr() as u64,
        elf_len: image.len() as u32,
        cert_len: cert.len() as u32,
        manifest_len: manifest.len() as u32,
        trailer_len: trailer.len() as u32,
    };
    let mut out = CapsuleVerifySummary::zeroed();
    match mk_capsule_verify(&req, &mut out) {
        0 => Ok(out),
        -13 => Err("rejected by verification"),
        -22 => Err("malformed artifact or manifest"),
        _ => Err("verification could not run"),
    }
}
