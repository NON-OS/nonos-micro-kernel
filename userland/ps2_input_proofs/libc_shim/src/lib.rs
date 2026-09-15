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

//! What the included driver files take from `nonos_libc`, answered by a
//! model the test attaches.
//!
//! Port I/O is the interesting half. The driver never touches the i8042
//! except through `mk_pio_read` and `mk_pio_write`, so a controller model
//! behind those two calls sees every byte in the order the driver sends it
//! and answers in the same thread, before the call returns. Nothing here
//! depends on a second thread winning a race.

mod broker;
mod port;

pub use broker::{
    acked, mk_debug, mk_device_claim, mk_device_list, mk_device_release, mk_irq_ack,
    mk_irq_bind, mk_pio_grant, present, DeviceRecord, IrqBindOut, PioGrantOut, BUS_KIND_ACPI,
    PIO_GRANT,
};
pub use port::{attach, mk_pio_read, mk_pio_release, mk_pio_write, Attached, Port};
