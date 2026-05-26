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
mod ack_irq;
mod drain_events;
mod dcbaa_slot;
mod get_config_descriptor;
mod get_device_descriptor;
mod halt;
mod issue_address_device;
mod issue_bulk_transfer;
mod issue_configure_endpoint;
mod issue_control_transfer;
mod issue_disable_slot;
mod issue_enable_slot;
mod issue_interrupt_in;
mod issue_noop_and_wait;
mod layout;
mod poll_interrupt_in;
mod program_command_ring;
mod program_dcbaa;
mod program_event_ring;
mod refuse_unsupported;
mod reset;
mod reset_port;
mod ring_doorbell;
mod scratchpad;
mod start;
mod wait_cnr_clear;
mod wait_command_completion;
mod wait_transfer_completion;
mod wait_hc_running;
pub use ack_irq::ack_irq;
pub use drain_events::drain_events;
pub use dcbaa_slot::{clear_dcbaa_slot, set_dcbaa_slot};
pub use get_config_descriptor::{get_config_descriptor, CONFIG_DESCRIPTOR_MAX};
pub use get_device_descriptor::{get_device_descriptor, DEVICE_DESCRIPTOR_LEN};
pub use halt::halt;
pub use issue_address_device::issue_address_device;
pub use issue_bulk_transfer::issue_bulk_transfer;
pub use issue_configure_endpoint::issue_configure_endpoint;
pub use issue_control_transfer::issue_control_transfer;
pub use issue_disable_slot::issue_disable_slot;
pub use issue_enable_slot::issue_enable_slot;
pub use issue_interrupt_in::issue_interrupt_in;
pub use issue_noop_and_wait::issue_noop_and_wait;
pub use layout::ControllerLayout;
pub use poll_interrupt_in::{poll_interrupt_in, IntrPoll};
pub use program_command_ring::program_command_ring;
pub use program_dcbaa::program_dcbaa;
pub use program_event_ring::program_event_ring;
pub use refuse_unsupported::refuse_unsupported;
pub use reset::reset;
pub use reset_port::reset_port;
pub use scratchpad::Scratchpads;
pub use start::start;
pub use wait_cnr_clear::wait_cnr_clear;
pub use wait_hc_running::wait_hc_running;