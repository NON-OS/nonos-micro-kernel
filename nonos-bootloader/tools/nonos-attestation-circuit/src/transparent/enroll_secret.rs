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

use super::constants::DOM_ENROLL;
use super::derive_scalar::derive_scalar;
use super::types::EnrolledSecret;

pub fn enroll_secret(label: &[u8], seed: &[u8]) -> EnrolledSecret {
    let mut sx = Vec::from(seed);
    sx.extend_from_slice(label);
    sx.push(0);
    let mut sr = Vec::from(seed);
    sr.extend_from_slice(label);
    sr.push(1);
    EnrolledSecret { x: derive_scalar(DOM_ENROLL, &sx), r: derive_scalar(DOM_ENROLL, &sr) }
}
