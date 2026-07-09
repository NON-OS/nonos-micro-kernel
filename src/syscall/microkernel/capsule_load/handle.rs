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

use super::copy::read_blob;
use super::errno::load_errno;
use super::request::CapsuleLoadRequest;
use crate::kernel_core::process_spawn::capsule_spawn::{
    load_capsule_from_vfs, AttestedParent, CapsuleArtifacts,
};
use crate::syscall::microkernel::errnos::ERRNO_FAULT;

// Load and spawn a capsule whose artifacts were read from the store by the
// installer and passed in by pointer. Returns the new pid or a negative errno.
// The service name and endpoints come from the signed manifest; every
// signature, manifest, and attestation check lives behind load_capsule_from_vfs.
pub fn sys_capsule_load(req_ptr: u64) -> i64 {
    match run(req_ptr) {
        Ok(pid) => pid,
        Err(errno) => errno,
    }
}

fn run(req_ptr: u64) -> Result<i64, i64> {
    let size = core::mem::size_of::<CapsuleLoadRequest>();
    if crate::usercopy::validate_user_read(req_ptr, size).is_err() {
        return Err(ERRNO_FAULT);
    }
    let req: CapsuleLoadRequest =
        crate::usercopy::read_user_value(req_ptr).map_err(|_| ERRNO_FAULT)?;
    let artifacts = CapsuleArtifacts {
        elf: read_blob(req.elf_ptr, req.elf_len)?,
        cert: read_blob(req.cert_ptr, req.cert_len)?,
        manifest: read_blob(req.manifest_ptr, req.manifest_len)?,
        trailer: read_blob(req.trailer_ptr, req.trailer_len)?,
    };
    let args = if req.args_len == 0 {
        alloc::vec::Vec::new()
    } else {
        read_blob(req.args_ptr, req.args_len)?
    };
    let on_behalf_of = resolve_on_behalf_of(req.on_behalf_of);
    let pid = load_capsule_from_vfs(artifacts, req.requested_caps, &args, on_behalf_of)
        .map_err(load_errno)?;
    Ok(pid as i64)
}

// `on_behalf_of` in the request is only honored when the calling process
// holds `Capability::SpawnBroker` (the installer, and no ordinary capsule)
// and the attributed pid is still live; both checks live in
// `AttestedParent::attest`, the sole constructor of the attested-parent
// newtype. A caller that fails either check gets the default caller-as-parent
// attribution, exactly as if the field were zero; this never turns into a
// hard error, since a forged, stale, or dead field must fail closed, not
// surface a capability probe.
fn resolve_on_behalf_of(raw: u32) -> Option<AttestedParent> {
    AttestedParent::attest(raw)
}
