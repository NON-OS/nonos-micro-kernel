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

use super::constants::{DOM_PEDERSEN, GENERATOR_H_LABEL};
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;

pub fn generator_h() -> EdwardsPoint {
    let cofactor = Scalar::from(8u64);
    let mut counter = 0u32;
    loop {
        let mut h = blake3::Hasher::new();
        h.update(DOM_PEDERSEN);
        h.update(GENERATOR_H_LABEL);
        h.update(&counter.to_le_bytes());
        if let Some(p) = CompressedEdwardsY(*h.finalize().as_bytes()).decompress() {
            let cleared = p * cofactor;
            if cleared != EdwardsPoint::identity() {
                return cleared;
            }
        }
        counter = counter.wrapping_add(1);
    }
}
