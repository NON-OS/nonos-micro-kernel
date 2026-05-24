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

mod border;
mod close_button;
mod hit_test;
mod metrics;
mod titlebar;

pub use border::draw_border;
pub use close_button::{close_button_rect, draw_close_button};
pub use hit_test::{hit_test, DecorationHit};
pub use metrics::{
    BORDER_PX, CLOSE_BUTTON_SIZE, TITLEBAR_HEIGHT, TITLEBAR_PADDING, TITLE_TEXT_Y,
};
pub use titlebar::draw_titlebar;
