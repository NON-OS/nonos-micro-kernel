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

//! How a config-space access actually reaches the bus.
//!
//! There are two ways, and which one is available is a property of the
//! machine, not of the architecture. The original PC mechanism writes an
//! address to port 0xCF8 and reads the answer from 0xCFC, which needs an I/O
//! port space and so exists only on a PC. Enhanced Configuration Access maps
//! the whole of config space into physical memory, one 4 KiB page per
//! function, and is the only mechanism an arm64 machine has; a PC with an
//! MCFG table has it too, and there it is both faster and the only way to
//! reach the extended registers above offset 0xFF.
//!
//! ECAM is preferred whenever the platform has published a window. Reporting
//! a failure when neither mechanism is available is deliberate: config reads
//! answer all-ones for an absent device, so a silent fallback would enumerate
//! an empty bus and look like a machine with no hardware on it.

mod ecam;
mod legacy;

use super::super::error::{PciError, Result};

pub use ecam::set_ecam_window;

/// Where an access should go.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mechanism {
    Ecam,
    /// Port 0xCF8/0xCFC. Only ever present on x86_64.
    Legacy,
    None,
}

fn mechanism() -> Mechanism {
    if ecam::is_configured() {
        return Mechanism::Ecam;
    }
    if cfg!(target_arch = "x86_64") {
        return Mechanism::Legacy;
    }
    Mechanism::None
}

fn unreachable(bus: u8, device: u8, function: u8, offset: u16) -> PciError {
    PciError::ConfigAccessFailed { bus, device, function, offset }
}

/// Generate the six accessors. Each width is the same shape: pick the
/// mechanism, or report that config space cannot be reached at all.
macro_rules! access {
    ($read:ident, $write:ident, $ty:ty) => {
        pub(super) fn $read(bus: u8, device: u8, function: u8, offset: u16) -> Result<$ty> {
            match mechanism() {
                Mechanism::Ecam => Ok(ecam::$read(bus, device, function, offset)),
                Mechanism::Legacy => Ok(legacy::$read(bus, device, function, offset)),
                Mechanism::None => Err(unreachable(bus, device, function, offset)),
            }
        }

        pub(super) fn $write(
            bus: u8,
            device: u8,
            function: u8,
            offset: u16,
            value: $ty,
        ) -> Result<()> {
            match mechanism() {
                Mechanism::Ecam => {
                    ecam::$write(bus, device, function, offset, value);
                    Ok(())
                }
                Mechanism::Legacy => {
                    legacy::$write(bus, device, function, offset, value);
                    Ok(())
                }
                Mechanism::None => Err(unreachable(bus, device, function, offset)),
            }
        }
    };
}

access!(read8, write8, u8);
access!(read16, write16, u16);
access!(read32, write32, u32);

pub(in crate::drivers::pci::config) use legacy::config_address;
