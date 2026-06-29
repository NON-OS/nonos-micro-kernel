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

pub fn cert_valid_now(cert: &[u8], now: u64) -> bool {
    let Some(val) = validity(cert) else {
        return false;
    };
    let Some((t1, v1, e1)) = super::der_tlv::der_tlv(val, 0) else {
        return false;
    };
    let Some((t2, v2, e2)) = super::der_tlv::der_tlv(val, e1) else {
        return false;
    };
    let Some(from) = super::cert_time_value::cert_time_value(t1, &val[v1..e1]) else {
        return false;
    };
    let Some(until) = super::cert_time_value::cert_time_value(t2, &val[v2..e2]) else {
        return false;
    };
    from <= now && now <= until
}

fn validity(cert: &[u8]) -> Option<&[u8]> {
    let (_, ov, _) = super::der_tlv::der_tlv(cert, 0)?;
    let (_, tv, _) = super::der_tlv::der_tlv(cert, ov)?;
    let mut o = tv;
    let (t0, _, e0) = super::der_tlv::der_tlv(cert, o)?;
    if t0 == 0xA0 {
        o = e0;
    }
    for _ in 0..3 {
        let (_, _, e) = super::der_tlv::der_tlv(cert, o)?;
        o = e;
    }
    let (t, v, e) = super::der_tlv::der_tlv(cert, o)?;
    if t != 0x30 {
        return None;
    }
    Some(&cert[v..e])
}
