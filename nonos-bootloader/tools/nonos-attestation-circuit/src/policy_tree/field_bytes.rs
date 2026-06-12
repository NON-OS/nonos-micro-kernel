// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use ark_ff::{BigInteger, PrimeField};

pub fn field_bytes<F: PrimeField>(value: &F) -> [u8; 32] {
    let raw = value.into_bigint().to_bytes_be();
    let mut out = [0u8; 32];
    let start = raw.len().saturating_sub(32);
    let bytes = &raw[start..];
    out[32 - bytes.len()..32].copy_from_slice(bytes);
    out
}
