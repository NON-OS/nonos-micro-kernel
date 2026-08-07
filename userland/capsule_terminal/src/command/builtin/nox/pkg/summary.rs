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

pub(super) struct PkgSummary {
    pub namespace: Vec<u8>,
    pub caps: u64,
    pub tier: u8,
    pub digest: [u8; 32],
}

// Reply payload: digest[32] | caps u64 LE | tier u8 | name_len u8 | name |
// ns_len u8 | ns. Hand-synced with `pkg_query::summary_payload`. The `name`
// field is the service endpoint, which is not what the package installs
// under, so it is stepped over rather than kept; `slug` derives the real
// installed name from the namespace.
pub(super) fn decode(p: &[u8]) -> Option<PkgSummary> {
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
    Some(PkgSummary { namespace: p[ns_at + 1..ns_at + 1 + ns_len].to_vec(), caps, tier, digest })
}

// The installed name is the last dot-separated segment of the namespace
// (`com.example.gui_demo` -> `gui_demo`), matching the installer's own
// `install_name`. This is the name the user later launches and removes.
pub(super) fn slug(ns: &[u8]) -> &[u8] {
    match ns.iter().rposition(|&b| b == b'.') {
        Some(i) => &ns[i + 1..],
        None => ns,
    }
}
