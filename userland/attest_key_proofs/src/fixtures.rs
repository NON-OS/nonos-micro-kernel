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

//! A TPM2_CreatePrimary response, built the way a part answers.

/// Header, handle and parameter size come before `outPublic`.
pub const OUT_PUBLIC_AT: usize = 18;

/// A TPM2_CreatePrimary response: header, handle, parameter size, then a
/// TPM2B_PUBLIC in the shape the kernel's template asks for.
pub fn response(curve: u16, scheme: u16, x_len: u16, y_len: u16) -> Vec<u8> {
    let mut pub_area = Vec::new();
    for v in [0x0023u16, 0x000B] {
        pub_area.extend_from_slice(&v.to_be_bytes());
    }
    pub_area.extend_from_slice(&0x0005_0072u32.to_be_bytes());
    pub_area.extend_from_slice(&0u16.to_be_bytes());
    pub_area.extend_from_slice(&0x0010u16.to_be_bytes());
    pub_area.extend_from_slice(&scheme.to_be_bytes());
    if scheme == 0x0018 {
        pub_area.extend_from_slice(&0x000Bu16.to_be_bytes());
    }
    pub_area.extend_from_slice(&curve.to_be_bytes());
    pub_area.extend_from_slice(&0x0010u16.to_be_bytes());
    for (len, fill) in [(x_len, 0xAAu8), (y_len, 0xBB)] {
        pub_area.extend_from_slice(&len.to_be_bytes());
        pub_area.extend(core::iter::repeat_n(fill, len as usize));
    }
    let mut r = vec![0x80, 0x02];
    r.extend_from_slice(&0u32.to_be_bytes());
    r.extend_from_slice(&0u32.to_be_bytes());
    r.extend_from_slice(&0x8000_0000u32.to_be_bytes());
    r.extend_from_slice(&((2 + pub_area.len()) as u32).to_be_bytes());
    r.extend_from_slice(&(pub_area.len() as u16).to_be_bytes());
    r.extend_from_slice(&pub_area);
    r.extend_from_slice(&[0u8; 8]);
    r
}
