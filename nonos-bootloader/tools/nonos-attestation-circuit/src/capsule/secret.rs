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

use super::hex32::hex32;
use curve25519_dalek::scalar::Scalar;
use std::fs;
use std::path::Path;

pub fn read_secret(path: &Path, label: &str) -> Result<(usize, Scalar, Scalar), String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    for (index, line) in text.lines().enumerate() {
        let mut parts = line.split_whitespace();
        if parts.next() != Some(label) {
            continue;
        }
        let x = hex32("secret_x", parts.next().ok_or("secret_x missing")?)?;
        let r = hex32("secret_r", parts.next().ok_or("secret_r missing")?)?;
        let x = Option::<Scalar>::from(Scalar::from_canonical_bytes(x))
            .ok_or("secret_x noncanonical")?;
        let r = Option::<Scalar>::from(Scalar::from_canonical_bytes(r))
            .ok_or("secret_r noncanonical")?;
        return Ok((index, x, r));
    }
    Err("capsule label not enrolled".into())
}
