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

//! Decode the pkg-query summary payload:
//! digest[32] | caps u64 LE | tier u8 | name_len u8 | name | ns_len u8 | ns.
//! Hand-synced with the installer's `pkg_query::summary_payload` and the
//! terminal's `nox/pkg/summary.rs`. The payload's `name` is the capsule's
//! service endpoint, not what the package installs under, so it is stepped
//! over rather than kept: the only name a consent prompt may show is the slug.

use alloc::vec::Vec;

/// Tier 1 is an enrolled capsule, tier 2 a publisher-signed one, matching the
/// kernel's encoding.
pub struct PkgSummary {
    pub slug: Vec<u8>,
    pub namespace: Vec<u8>,
    pub caps: u64,
    pub tier: u8,
    pub digest: [u8; 32],
}

pub(super) fn decode_summary(p: &[u8]) -> Option<PkgSummary> {
    if p.len() < 42 {
        return None;
    }
    let digest: [u8; 32] = p[0..32].try_into().ok()?;
    let caps = u64::from_le_bytes(p[32..40].try_into().ok()?);
    let tier = p[40];
    let ns_at = 42usize.checked_add(p[41] as usize)?;
    if p.len() < ns_at + 1 {
        return None;
    }
    let ns_len = p[ns_at] as usize;
    if p.len() < ns_at + 1 + ns_len {
        return None;
    }
    let namespace = p[ns_at + 1..ns_at + 1 + ns_len].to_vec();
    Some(PkgSummary { slug: slug(&namespace).to_vec(), namespace, caps, tier, digest })
}

/// The installed name is the last dot-separated segment of the namespace
/// (`com.example.gui_demo` -> `gui_demo`), matching the installer's own
/// `install_name`. This is the name the user later launches and removes.
fn slug(ns: &[u8]) -> &[u8] {
    match ns.iter().rposition(|&b| b == b'.') {
        Some(i) => &ns[i + 1..],
        None => ns,
    }
}
