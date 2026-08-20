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

//! Runtime TPM support.
//!
//! The bootloader has its own TPM stack for measuring the boot chain. This one
//! exists because a quote must be taken while the machine is running, against
//! a nonce that did not exist at boot, and by then the bootloader is gone.

pub mod ak;
pub mod crb;
pub mod error;
pub mod quote;
