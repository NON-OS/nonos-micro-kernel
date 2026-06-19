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

pub mod constants;
pub mod firmware;
pub mod framebuffer;
pub mod handoff;
pub mod info;
pub mod memory;
pub mod security;

pub use constants::{flags, pixel_format, HANDOFF_MAGIC, HANDOFF_VERSION};
pub use constants::{truncate_cmdline, validate_cmdline_len};
pub use firmware::{FirmwareEntry, FirmwareHandoff, FirmwareType, MAX_FIRMWARE_ENTRIES};
pub use framebuffer::FramebufferInfo;
pub use handoff::BootHandoffV1;
pub use info::{AcpiInfo, Module, Modules, SmbiosInfo, Timing};
pub use memory::{memory_type, MemoryMap, MemoryMapEntry};
pub use security::{Measurements, RngSeed, ZkAttestation};

pub const MAX_CMDLINE: usize = constants::MAX_CMDLINE_LEN;
