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

use nonos_libc::mk_service_lookup;

use super::consts::PAYMENT_SERVICE;

// A package whose verified namespace ends in a builtin's slug would answer in
// that builtin's place. The live registry is the only name list that cannot
// drift from what is actually running, so it is what the slug is tested
// against. The probe covers the two conventional spellings: `app.<slug>`, which
// packaged GUI apps use, and the bare `<slug>` the infrastructure services (vfs,
// compositor, installer) register under. The registered name comes from the
// manifest's Service endpoint rather than the slug, so this is a convention and
// not an invariant; a package that declares some other endpoint is caught
// instead by `register_endpoint` refusing a duplicate name.
pub fn service_taken(slug: &[u8]) -> bool {
    let mut prefixed = Vec::with_capacity(4 + slug.len());
    prefixed.extend_from_slice(b"app.");
    prefixed.extend_from_slice(slug);
    resolves(&prefixed) || resolves(slug)
}

fn resolves(name: &[u8]) -> bool {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) == 0
}

pub fn payment_port() -> Option<u32> {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc =
        mk_service_lookup(PAYMENT_SERVICE.as_ptr(), PAYMENT_SERVICE.len(), &mut port, &mut pid);
    if rc != 0 {
        return None;
    }
    Some(port)
}
