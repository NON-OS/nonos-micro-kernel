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

mod arm_waiter;
mod clear_waiter;
mod consumer_ready;
mod diag;
mod drain;
mod drains;
mod post;
mod ring;
mod seq;

pub use arm_waiter::arm_input_waiter;
pub use clear_waiter::clear_input_waiter;
#[cfg(feature = "input-probe-inject")]
pub use consumer_ready::consumer_ready;
pub use diag::input_diag;
pub use drain::drain_input;
pub use drains::input_drains;
pub use post::post_input;
pub use ring::KIND_DIAG_BASE;
pub use seq::input_seq;
