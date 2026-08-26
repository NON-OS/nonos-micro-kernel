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

use nonos_app_skeleton::paint::PaintBuffer;

use crate::app::state::VideoApp;
use crate::ui::frame::{region, sidebar, topbar};
use crate::ui::screen::Route;
use crate::ui::theme;

pub fn hint(route: Route) -> &'static str {
    match route {
        Route::Settings => "Search settings",
        Route::Files => "Search this folder",
        Route::Playlists => "Search playlists",
        _ => "Search videos, playlists, folders",
    }
}

pub fn has_tools(route: Route) -> bool {
    matches!(route, Route::Home | Route::Library | Route::Files)
}

pub fn paint_route(fb: &mut PaintBuffer, app: &VideoApp) {
    for px in fb.pixels.iter_mut() {
        *px = theme::APP_BG;
    }
    let (w, h) = (fb.width, fb.height);
    let route = app.route();
    sidebar::paint_sidebar(fb, route, app.browse.items.len());
    topbar::paint_search(fb, w, h, hint(route), &app.browse.query);
    if has_tools(route) {
        let sort = "Sort by name";
        let filter = "All videos";
        topbar::paint_tools(fb, w, h, filter, sort, app.browse.grid);
    }
    let body = region::body(w, h);
    match route {
        Route::Home => super::home::paint(fb, app, body),
        Route::Library => super::library::paint(fb, app, body),
        Route::Playlists => super::playlists::paint(fb, app, body),
        Route::Files => super::files::paint(fb, app, body),
        Route::Settings => super::settings::paint(fb, app, body),
        Route::Details => super::details::paint(fb, app, body),
        Route::Player => {}
    }
}
