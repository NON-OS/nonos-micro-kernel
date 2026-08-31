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

mod accessory;
mod frame_rect;
mod hit_test;
mod metrics;
mod palette;
mod rect;
mod titlebar;
mod traffic_lights;

pub use accessory::accessory_rect;
pub use frame_rect::{content_rect, frame_rect, light_rect, margin, radius, titlebar_rect};
pub use hit_test::{hit_test, DecorationHit};
pub use metrics::{
    ACCESSORY_INSET, ACCESSORY_PAD_Y, BORDER_PX, FRAME_RADIUS, HAIRLINE_PX, LIGHT_D, LIGHT_GAP,
    LIGHT_HIT_PAD, LIGHT_INSET, SHADOW_MARGIN, TITLEBAR_H, TITLE_PX,
};
pub use palette::{
    FRAME_BG, FRAME_BORDER, HAIRLINE, LIGHT_CLOSE, LIGHT_GLYPH, LIGHT_MAXIMIZE, LIGHT_MINIMIZE,
    SHADOW, TITLE_TEXT, TRANSPARENT,
};
pub use rect::Rect;
pub use titlebar::draw_frame;
pub use traffic_lights::draw_traffic_lights;
