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

#![no_std]
#![no_main]

use nonos_appkit::{Button, Label, Panel, Theme};
use nonos_sdk::prelude::*;

fn app() {
    let theme = Theme::dark();
    let root = Panel::new()
        .label(Label::new(Rect { x: 16, y: 16, w: 300, h: 8 }, "NONOS App Kit", theme.foreground))
        .button(Button::new(
            Rect { x: 16, y: 40, w: 120, h: 28 },
            "Launch",
            theme.button_bg,
            theme.button_fg,
            theme.accent,
        ));
    App::new("App Kit Demo").size(480, 320).background(theme.background).run(root);
}

// Widgets and a window; it needs nothing else.
sdk_main!(app, caps: [WINDOW]);
