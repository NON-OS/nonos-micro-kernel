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


//! The client's scene, and the NONOS surface it ends up on.

use alloc::vec::Vec;

use super::state::{Buffer, Pool, Surface};

pub struct Scene {
    pub pools: Vec<Pool>,
    pub buffers: Vec<Buffer>,
    pub surfaces: Vec<Surface>,
    /// Serial for configure events, which a client echoes back.
    pub serial: u32,
    /// The NONOS surface the client's pixels end up on.
    pub out: Option<u64>,
    /// This capsule's own copy of those pixels, which the surface
    /// descriptor points at.
    pub pixels: Vec<u8>,
    pub pointer: Option<u32>,
    pub keyboard: Option<u32>,
    pub pointer_entered: bool,
    pub keyboard_entered: bool,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pools: Vec::new(),
            buffers: Vec::new(),
            surfaces: Vec::new(),
            serial: 1,
            out: None,
            pixels: Vec::new(),
            pointer: None,
            keyboard: None,
            pointer_entered: false,
            keyboard_entered: false,
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        self.serial += 1;
        self.serial
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}
