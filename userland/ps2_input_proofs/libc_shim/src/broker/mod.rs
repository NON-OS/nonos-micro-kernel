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

//! The broker half of the shim.

mod calls;
mod table;
mod types;

pub use calls::{
    mk_debug, mk_device_claim, mk_device_release, mk_irq_ack, mk_irq_bind, mk_pio_grant,
};
pub use table::{acked, mk_device_list, present};
pub use types::{DeviceRecord, IrqBindOut, PioGrantOut, BUS_KIND_ACPI, IRQ_GRANT_BASE, PIO_GRANT};
