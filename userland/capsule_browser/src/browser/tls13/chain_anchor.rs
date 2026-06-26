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

pub fn chain_anchor(body: &[u8]) -> bool {
    if super::cert_count::cert_count(body) < 2 {
        return false;
    }
    let Some(cert) = super::last_cert::last_cert(body) else { return false };
    let Some(spki) = super::cert_spki::cert_spki(cert) else { return false };
    let Some(hash) = super::hash_sha256::hash_sha256(spki) else { return false };
    super::gts_r4_anchor::gts_r4_anchor(&hash)
}
