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
use super::absorb::absorb;
use super::drainer::Drainer;
use super::read_port::read_port;
use crate::constants::{DATA_OFFSET, STATUS_OFFSET};
use crate::constants::{STATUS_AUX_DATA, STATUS_OUTPUT_FULL, STATUS_PARITY, STATUS_TIMEOUT};
use crate::mouse::{MouseParser, MouseRing};
use crate::ring::Ring;
const MAX_BYTES_PER_DRAIN: u32 = 16;
pub fn drain(
    grant_id: u64,
    drainer: &mut Drainer,
    ring: &mut Ring,
    mouse: &mut MouseParser,
    mouse_ring: &mut MouseRing,
) {
    for _ in 0..MAX_BYTES_PER_DRAIN {
        let status = match read_port(grant_id, STATUS_OFFSET) {
            Some(v) => v,
            None => return,
        };
        // super::diag::signal_pio_ok(); // bring-up diagnostic, silenced
        if status & STATUS_OUTPUT_FULL == 0 {
            return;
        }
        if status & STATUS_PARITY != 0 {
            ring.parity_errors = ring.parity_errors.wrapping_add(1);
        }
        if status & STATUS_TIMEOUT != 0 {
            ring.timeout_errors = ring.timeout_errors.wrapping_add(1);
        }
        let byte = match read_port(grant_id, DATA_OFFSET) {
            Some(v) => v,
            None => return,
        };
        // super::diag::signal_byte(); // bring-up diagnostic, silenced
        if status & STATUS_AUX_DATA != 0 {
            mouse.absorb(byte, mouse_ring);
        } else {
            absorb(drainer, ring, byte);
        }
    }
}
