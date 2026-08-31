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

//! Dispatch a frame to the screen the window is currently showing. Each screen
//! owns its own painter and is responsible for filling the whole content rect.

use nonos_app_skeleton::PaintBuffer;

use super::activity_bar::paint_activity;
use super::app::Editor;
use super::home::paint_home;
use super::screen::Screen;
use super::settings::paint_settings;

impl Editor {
    pub(super) fn paint_shell(&mut self, fb: &mut PaintBuffer) {
        match self.screen {
            Screen::Editor => self.paint_editor_screen(fb),
            Screen::Home => paint_home(self, fb),
            Screen::Settings => paint_settings(self, fb),
        }
        if self.screen != Screen::Editor {
            paint_activity(fb, fb.height, self.screen, self.sidebar_open);
        }
    }
}
