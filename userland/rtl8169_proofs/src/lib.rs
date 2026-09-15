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

//! The rtl8169 bring-up, proved without the card.
//!
//! The `#[path]` includes pull in the shipping driver source, so these tests
//! run the code that boots. The register type is built from a base address,
//! so a window in host memory stands in for BAR0 and a device modelled from
//! the datasheet answers from it. The broker, DMA mapping and server halves
//! talk to the kernel rather than to registers and stay out.
//!
//! The property that earns this crate its place is the station address. The
//! driver draws one and programs it; the factory address burned into the part
//! is the one identifier an amnesic machine would otherwise announce to every
//! network it joins, and the driver's promise is that it never falls back to
//! it. That is checked here with the entropy source switched off.

#[path = "../../capsule_driver_rtl8169/src/constants/mod.rs"]
pub mod constants;
pub mod init;
#[path = "../../capsule_driver_rtl8169/src/queue/mod.rs"]
pub mod queue;
#[path = "../../capsule_driver_rtl8169/src/regs.rs"]
pub mod regs;
pub mod setup;

#[cfg(test)]
mod tests;
