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
mod pnp;
mod ports;
mod status;
pub use pnp::{PNP_DEVICE_PS2_AUX, PNP_DEVICE_PS2_KBD, PNP_VENDOR_PS2_AUX, PNP_VENDOR_PS2_KBD};
pub use ports::{
    CONFIG_AUX_DISABLE, CONFIG_IRQ1, CONFIG_IRQ12, CTL_ENABLE_AUX, CTL_READ_CONFIG, CTL_WRITE_AUX,
    CTL_WRITE_CONFIG, DATA_OFFSET, KBD_ENABLE_SCANNING, MOUSE_ACK, MOUSE_ENABLE_REPORTING,
    MOUSE_SET_DEFAULTS, RING_CAPACITY, STATUS_OFFSET,
};
pub use status::{
    STATUS_AUX_DATA, STATUS_INPUT_FULL, STATUS_OUTPUT_FULL, STATUS_PARITY, STATUS_TIMEOUT,
};