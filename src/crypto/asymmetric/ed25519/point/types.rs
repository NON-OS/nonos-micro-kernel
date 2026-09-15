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

pub(crate) use crate::crypto::asymmetric::ed25519::field::Fe;

#[derive(Copy, Clone)]
pub(crate) struct GeP3 {
    pub(crate) x: Fe,
    pub(crate) y: Fe,
    pub(crate) z: Fe,
    pub(crate) t: Fe,
}

#[derive(Copy, Clone)]
pub(crate) struct GeP2 {
    pub(crate) x: Fe,
    pub(crate) y: Fe,
    pub(crate) z: Fe,
}

#[derive(Copy, Clone)]
pub(crate) struct GeCached {
    pub(crate) y_plus_x: Fe,
    pub(crate) y_minus_x: Fe,
    pub(crate) z: Fe,
    pub(crate) t2d: Fe,
}

impl GeP3 {
    pub(crate) fn identity() -> Self {
        Self { x: Fe::zero(), y: Fe::one(), z: Fe::one(), t: Fe::zero() }
    }

    pub(crate) fn to_p2(self) -> GeP2 {
        GeP2 { x: self.x, y: self.y, z: self.z }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct GeP1P1 {
    pub(crate) x: Fe,
    pub(crate) y: Fe,
    pub(crate) z: Fe,
    pub(crate) t: Fe,
}

pub(crate) const D: Fe = Fe([
    -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719, -18696448,
    -12055116,
]);

pub(crate) const D2: Fe = Fe([
    -21827220, 27714826, -30745222, 13898782, 229458, -17575632, -12551816, -6495438, -37392896,
    -24110232,
]);
