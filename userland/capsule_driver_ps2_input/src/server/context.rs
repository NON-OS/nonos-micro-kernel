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
use crate::mouse::{MouseParser, MouseRing};
use crate::poll::Drainer;
use crate::ring::Ring;
use crate::setup::Driver;
pub struct Context {
    pub driver: Driver,
    pub ring: Ring,
    pub drainer: Drainer,
    pub mouse: MouseParser,
    pub mouse_ring: MouseRing,
    pub last_kbd_seq: u64,
    pub last_aux_seq: u64,
}
impl Context {
    pub fn new(driver: Driver) -> Self {
        Self {
            driver,
            ring: Ring::new(),
            drainer: Drainer::new(),
            mouse: MouseParser::new(),
            mouse_ring: MouseRing::new(),
            last_kbd_seq: 0,
            last_aux_seq: 0,
        }
    }
}
