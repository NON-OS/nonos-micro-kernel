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

/// What the CPU underneath this kernel can produce, latched at boot.
pub(crate) struct EntropySource {
    /// A conditioned DRBG tap is present (RDRAND, RNDR).
    pub(crate) random_available: bool,
    /// A reseeded entropy tap is present (RDSEED, RNDRRS).
    pub(crate) entropy_available: bool,
}

pub(crate) static mut ENTROPY_SOURCE: EntropySource =
    EntropySource { random_available: false, entropy_available: false };
