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

pub fn ecdsa_sig_raw(der: &[u8], width: usize, out: &mut [u8]) -> bool {
    let Some((0x30, val, end)) = super::der_tlv::der_tlv(der, 0) else { return false };
    let Some((0x02, r0, r1)) = super::der_tlv::der_tlv(der, val) else { return false };
    let Some((0x02, s0, s1)) = super::der_tlv::der_tlv(der, r1) else { return false };
    if end != der.len() || s1 != end || out.len() != width * 2 {
        return false;
    }
    write_int(&der[r0..r1], &mut out[..width]) && write_int(&der[s0..s1], &mut out[width..])
}

fn write_int(src: &[u8], dst: &mut [u8]) -> bool {
    let src = if src.len() > 1 && src[0] == 0 { &src[1..] } else { src };
    if src.len() > dst.len() {
        return false;
    }
    dst.fill(0);
    let start = dst.len() - src.len();
    dst[start..].copy_from_slice(src);
    true
}
