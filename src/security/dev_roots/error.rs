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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnrolError {
    /// The caller does not hold `EnrolDevRoot`.
    Denied,
    /// A running capsule went unrecorded earlier, so the machine cannot
    /// describe itself completely and must not add to what it runs.
    RegistryIncomplete,
    /// An all-zero root, which is what an uninitialised buffer looks like.
    EmptyRoot,
    /// Every slot is taken. Enrolment is deliberately scarce.
    NoSlots,
    /// No pending request, or the wrong code. The two are one error on
    /// purpose: telling a caller which it was would say whether somebody
    /// else's enrolment is in flight.
    NotConfirmed,
}

impl EnrolError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "capability EnrolDevRoot not held",
            Self::RegistryIncomplete => "attestation registry incomplete",
            Self::EmptyRoot => "developer root is all zero",
            Self::NoSlots => "no developer root slots remain",
            Self::NotConfirmed => "no pending enrolment, or wrong code",
        }
    }

    /// Negative errno for the syscall boundary. `Denied` and the rest are kept
    /// distinct: a developer whose enrolment failed needs to know whether to
    /// fix their manifest or reboot.
    pub const fn to_errno(self) -> i64 {
        match self {
            Self::Denied => -1,
            Self::RegistryIncomplete => -5,
            Self::EmptyRoot => -22,
            Self::NoSlots => -28,
            Self::NotConfirmed => -1,
        }
    }
}
