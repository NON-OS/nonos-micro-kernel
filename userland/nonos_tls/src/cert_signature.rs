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

pub fn cert_signature(cert: &[u8]) -> Option<&[u8]> {
    let (_, val, _) = super::der_tlv::der_tlv(cert, 0)?;
    let (_, _, tbs_end) = super::der_tlv::der_tlv(cert, val)?;
    let (_, _, alg_end) = super::der_tlv::der_tlv(cert, tbs_end)?;
    let (tag, sig_val, sig_end) = super::der_tlv::der_tlv(cert, alg_end)?;
    if tag == 0x03 && sig_val < sig_end && cert[sig_val] == 0 {
        Some(&cert[sig_val + 1..sig_end])
    } else {
        None
    }
}
