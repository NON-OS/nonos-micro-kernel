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

//! Timer-interrupt heartbeat. A small square in the top-left corner whose
//! color advances one step per second of ticks, painted straight to the
//! framebuffer from the timer ISR. It bypasses the compositor entirely, so
//! it keeps blinking even under the desktop: if this square animates on a
//! machine with no serial console, the LAPIC timer interrupt is firing; if
//! it is frozen, no timer interrupt is being delivered and every
//! tick-driven wakeup (preemption, sleeping capsules, the clock) is dead.
//! Bring-up diagnostic, compiled only under NONOS_FBCONSOLE.

mod consts;
mod device_irq_blip;
mod draw_bar;
mod input_activity_bars;
mod input_drain_blip;
mod input_post_blip;
mod on_tick;
mod paint_blip;

pub use device_irq_blip::device_irq_blip;
pub use input_activity_bars::input_activity_bars;
pub use input_drain_blip::input_drain_blip;
pub use input_post_blip::input_post_blip;
pub use on_tick::on_tick;
