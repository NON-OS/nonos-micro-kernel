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

//! Host proofs for the input decoders. Each `#[path]` include pulls in the real
//! driver source so the tests pin production decode logic, not a copy. The
//! module names match what the included files reach for via `super::`.

// PS/2 mouse packet decoder and the event type it returns.
#[path = "../../capsule_driver_ps2_input/src/mouse/event.rs"]
pub mod event;
#[path = "../../capsule_driver_ps2_input/src/mouse/packet.rs"]
pub mod packet;

// i2c-hid touchpad report decoder and its sample type.
#[path = "../../capsule_driver_i2c_hid/src/input/parse_report.rs"]
pub mod parse_report;
#[path = "../../capsule_driver_i2c_hid/src/input/sample.rs"]
pub mod sample;

// The HID report-descriptor parser and the absolute-touch decoder, included
// flat so each file's `super::` references resolve to the crate root.
#[path = "../../capsule_driver_i2c_hid/src/hid/report_desc/decode.rs"]
pub mod decode;
#[path = "../../capsule_driver_i2c_hid/src/hid/report_desc/layout.rs"]
pub mod layout;
#[path = "../../capsule_driver_i2c_hid/src/hid/report_desc/parse.rs"]
pub mod parse;
#[path = "../../capsule_driver_i2c_hid/src/hid/report_desc/read_bits.rs"]
pub mod read_bits;

// The touchpad gesture state machine (tap, move, two-finger scroll).
#[path = "../../capsule_driver_i2c_hid/src/input/gesture.rs"]
pub mod gesture;

// The compositor's damage accumulator, whose coalescing must never drop a
// damaged pixel or the screen tears.
#[path = "../../compositor/src/state/damage.rs"]
pub mod damage;

#[cfg(test)]
mod damage_tests;
#[cfg(test)]
mod gesture_tests;
#[cfg(test)]
mod hid_touchpad_tests;
#[cfg(test)]
mod ps2_tests;
#[cfg(test)]
mod touchpad_tests;
