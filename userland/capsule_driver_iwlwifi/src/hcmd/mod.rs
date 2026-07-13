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

//! The host-command queue: how the driver hands commands and frames to the
//! alive firmware. `header` frames a command, `ring` tracks the TFD ring
//! indices, `tfd` fills a descriptor, `doorbell` tells the firmware a queue
//! advanced, and `send` ties them into the one operation the scan, auth and
//! assoc steps issue commands through.

pub mod doorbell;
pub mod header;
pub mod ring;
pub mod send;
pub mod tfd;
