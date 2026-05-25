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

use crate::catalog_client::fetch_image;
use crate::compositor_client::push_damage_commit;
use crate::paint::{decode_jpeg, paint_image};
use crate::state::Context;

pub fn apply(ctx: &mut Context, index: u8) -> bool {
    let catalog_port = match ctx.catalog_port {
        Some(p) => p,
        None => return false,
    };
    let bytes = match fetch_image(catalog_port, index as u32) {
        Some(b) => b,
        None => return false,
    };
    let img = match decode_jpeg(&bytes) {
        Some(d) => d,
        None => return false,
    };
    paint_image(ctx, &img);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height);
    true
}
