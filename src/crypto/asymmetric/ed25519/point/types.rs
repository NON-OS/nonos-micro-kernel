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
    pub(crate) X: Fe,
    pub(crate) Y: Fe,
    pub(crate) Z: Fe,
    pub(crate) T: Fe,
}

#[derive(Copy, Clone)]
pub(crate) struct GeP2 {
    pub(crate) X: Fe,
    pub(crate) Y: Fe,
    pub(crate) Z: Fe,
}

#[derive(Copy, Clone)]
pub(crate) struct GeCached {
    pub(crate) YplusX: Fe,
    pub(crate) YminusX: Fe,
    pub(crate) Z: Fe,
    pub(crate) T2d: Fe,
}

impl GeCached {
    pub(crate) fn identity() -> Self {
        Self { YplusX: Fe::one(), YminusX: Fe::one(), Z: Fe::one(), T2d: Fe::zero() }
    }
}

impl GeP3 {
    pub(crate) fn identity() -> Self {
        Self { X: Fe::zero(), Y: Fe::one(), Z: Fe::one(), T: Fe::zero() }
    }

    pub(crate) fn to_p2(&self) -> GeP2 {
        GeP2 { X: self.X, Y: self.Y, Z: self.Z }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct GeP1P1 {
    pub(crate) X: Fe,
    pub(crate) Y: Fe,
    pub(crate) Z: Fe,
    pub(crate) T: Fe,
}

pub(crate) const D: Fe = Fe([
    -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719, -18696448,
    -12055116,
]);

pub(crate) const D2: Fe = Fe([
    -21827220, 27714826, -30745222, 13898782, 229458, -17575632, -12551816, -6495438, -37392896,
    -24110232,
]);
