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

//! The menu bar across the top of the desktop: the real NØNOS logo and wordmark
//! on the left, and a live status cluster on the right holding the battery,
//! network, clock and last-notification state. Styled to match the dock so the
//! two bars read as one system.

mod background;
mod battery_glyph;
mod brand;
mod brand_hit;
mod metrics;
mod net_glyph;
mod notify_dot;
mod paint;
mod status;

pub(crate) use metrics::brand_right;

pub use brand_hit::brand_hit;
pub use paint::paint;
