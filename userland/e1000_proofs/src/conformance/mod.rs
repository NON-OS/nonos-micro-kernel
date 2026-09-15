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

//! The bring-up against a modelled part.
//!
//! The property that earns its place: the card is brought on the air under
//! a drawn station address and never under the one in its EEPROM. That
//! address is the one identifier an amnesic machine would otherwise announce
//! to every network it joins, and the driver's promise is that it fails
//! closed without entropy rather than falling back. Checked with the entropy
//! source switched off, and by a part watching the receive enable.

mod memory;
mod model;

mod address_tests;
mod bring_up_tests;
mod reset_tests;
mod ring_tests;
