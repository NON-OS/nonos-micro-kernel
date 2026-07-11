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

pub mod api;
pub mod ioapic;
pub mod ioapic_madt;
pub mod local;
pub mod local_calibrate;
pub mod vectors;

pub use api::{init, is_init, setup_keyboard_irq, setup_mouse_irq};
pub use ioapic::{disable_irq, enable_irq, init_ioapic, ioapic_set_irq};
pub use local::{
    eoi, init_local_apic, lapic_state, rebind_to_virt, setup_timer, stop_timer, LAPIC_PHYS_BASE,
    TIMER_VECTOR,
};
pub use vectors::*;
