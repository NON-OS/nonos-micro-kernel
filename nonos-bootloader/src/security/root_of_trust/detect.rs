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

//! Deciding what is anchoring this boot.

use super::level::RootOfTrust;

/// What is anchoring this boot's measurements and rollback floor.
///
/// The probe is the interface read the TPM driver already performs, which covers
/// a discrete part and the firmware ones alike: Intel PTT and AMD fTPM answer on
/// the same command-response interface as a chip on the bus, so a machine with
/// any of them comes back anchored. A machine with none is unanchored and says
/// so, which the caller turns into a refusal on a production image.
pub fn detect() -> RootOfTrust {
    if crate::hardware::tpm::is_tpm_available() {
        RootOfTrust::Tpm
    } else {
        RootOfTrust::Unanchored
    }
}
