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

use super::verify::Verified;
use crate::kernel_core::process_spawn::capsule_spawn::Tier;
use crate::security::capsule_manifest::EndpointKind;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CapsuleVerifySummary {
    pub caps: u64,
    pub tier: u8,
    pub name_len: u8,
    pub ns_len: u8,
    pub _pad: [u8; 5],
    pub name: [u8; 64],
    pub namespace: [u8; 64],
}

pub(super) fn fill(v: &Verified) -> CapsuleVerifySummary {
    let mut out = CapsuleVerifySummary {
        caps: v.install_caps,
        tier: match v.tier {
            Tier::Enrolled => 1,
            Tier::Publisher => 2,
        },
        name_len: 0,
        ns_len: 0,
        _pad: [0u8; 5],
        name: [0u8; 64],
        namespace: [0u8; 64],
    };
    let service = v
        .manifest
        .endpoints
        .iter()
        .find(|e| e.kind == EndpointKind::Service)
        .map(|e| e.name_str())
        .unwrap_or("");
    out.name_len = copy_str(&mut out.name, service);
    out.ns_len = copy_str(&mut out.namespace, v.manifest.namespace_str());
    out
}

fn copy_str(dst: &mut [u8; 64], s: &str) -> u8 {
    let n = core::cmp::min(s.len(), dst.len());
    dst[..n].copy_from_slice(&s.as_bytes()[..n]);
    n as u8
}
