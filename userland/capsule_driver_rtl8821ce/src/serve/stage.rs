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

//! How far the radio bring-up got.

/// How far the radio bring-up got. Reported to the panel so a failure is legible
/// on screen instead of the driver exiting and vanishing from the service table.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Fully up: firmware, MAC, PHY and the rings are ready; scanning works.
    Ready = 0,
    /// The device could not be claimed or its registers mapped.
    NotClaimed = 1,
    /// The power-on sequence never reached readiness.
    PowerFailed = 2,
    /// The register window read back dead after power-on.
    DeadMmio = 3,
    /// The firmware download did not complete.
    FirmwareFailed = 4,
    /// Bring-up succeeded but the transmit/receive DMA could not be mapped.
    NoDma = 5,
    /// The efuse never read back, so the PHY could not be configured for this
    /// board and the radio was left dark rather than programmed with guesses.
    EfuseFailed = 6,
    /// No entropy for a station address, so the radio was left dark rather than
    /// transmitting under a predictable one. Separate from `EfuseFailed` because
    /// both were once reported under that name, and a stage that names the wrong
    /// step sends every investigation to the wrong register.
    NoStationAddress = 7,
}
