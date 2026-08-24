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

pub fn chain_signatures(body: &[u8]) -> bool {
    if !super::chain_anchor::chain_anchor(body) {
        return false;
    }
    let Some(leaf) = super::cert_at::cert_at(body, 0) else { return false };
    let Some(we1) = super::cert_at::cert_at(body, 1) else { return false };
    let Some(gts_r4) = super::cert_at::cert_at(body, 2) else { return false };
    // Pinning the root is not enough on its own: any certificate that root
    // issued would otherwise serve as an intermediate and sign a forged leaf.
    nonos_tls::cert_is_ca(we1)
        && super::verify_leaf::verify_leaf(leaf, we1)
        && super::verify_intermediate::verify_intermediate(we1, gts_r4)
}
