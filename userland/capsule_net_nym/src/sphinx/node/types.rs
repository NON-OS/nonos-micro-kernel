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

use crate::sphinx::constants::{
    DESTINATION_ADDRESS_LENGTH, IDENTIFIER_LENGTH, NODE_ADDRESS_LENGTH,
};

/// A mix on the route: where to reach it and the key packets are sealed to.
#[derive(Clone, Copy)]
pub struct Node {
    pub address: [u8; NODE_ADDRESS_LENGTH],
    pub pub_key: [u8; 32],
}

/// Where the packet finally goes.
#[derive(Clone, Copy)]
pub struct Destination {
    pub address: [u8; DESTINATION_ADDRESS_LENGTH],
    pub identifier: [u8; IDENTIFIER_LENGTH],
}
