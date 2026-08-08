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

use crate::sphinx::constants::{DESTINATION_ADDRESS_LENGTH, IDENTIFIER_LENGTH, VERSION_LENGTH};

/// What the last hop is told. No address or delay: it forwards to nobody.
pub struct FinalRoutingInformation {
    pub flag: u8,
    pub version: [u8; VERSION_LENGTH],
    pub destination: [u8; DESTINATION_ADDRESS_LENGTH],
    pub identifier: [u8; IDENTIFIER_LENGTH],
}
