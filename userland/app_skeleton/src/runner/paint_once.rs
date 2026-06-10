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

use nonos_toolkit::decorations::DecorationHit;

use crate::app::{App, AppManifest};
use crate::clients::compositor;
use crate::setup::WindowBinding;

use super::paint_frame::paint;
use super::request_id::next;

pub(super) fn paint_once<A: App>(
    app: &mut A,
    manifest: &AppManifest,
    binding: &WindowBinding,
    compositor_port: u32,
    request_id: &mut u32,
) -> bool {
    paint(app, manifest, binding, DecorationHit::None);
    let rid = next(request_id);
    compositor::damage_commit(compositor_port, rid, binding.x, binding.y, binding.width, binding.height)
        .is_ok()
}
