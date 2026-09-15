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

//! Port IO, which does not exist on the host.
//!
//! The shipping file issues `mk_pio_read`/`mk_pio_write` syscalls. Nothing on
//! a development machine can answer those, so rather than return zero and let
//! a test read a fabricated device response as if it were real, both entry
//! points refuse. A window built with `Regs::mmio` never reaches here.

pub fn read(_grant: u64, _offset: usize, _width: u8) -> u32 {
    panic!("virtio_rng_proofs: a test reached the PIO transport; build the window with Regs::mmio");
}

pub fn write(_grant: u64, _offset: usize, _width: u8, _value: u32) {
    panic!("virtio_rng_proofs: a test reached the PIO transport; build the window with Regs::mmio");
}
