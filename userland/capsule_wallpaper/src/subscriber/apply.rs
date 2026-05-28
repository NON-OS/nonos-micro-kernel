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
use crate::debug;
use crate::paint::{decode_jpeg, paint_image};
use crate::state::Context;

pub fn apply(ctx: &mut Context, index: u8) -> bool {
    debug::marker(b"apply enter");
    let catalog_port = match ctx.catalog_port {
        Some(p) => p,
        None => {
            debug::marker(b"no catalog_port");
            return false;
        }
    };
    let bytes = match fetch_image(catalog_port, index as u32) {
        Some(b) => b,
        None => {
            debug::marker(b"fetch_image failed");
            return false;
        }
    };
    debug::marker(b"fetch_image ok");
    let img = match decode_jpeg(&bytes) {
        Some(d) => d,
        None => {
            debug::marker(b"decode_jpeg failed");
            return false;
        }
    };
    debug::marker(b"decode_jpeg ok");
    paint_image(ctx, &img);
    debug::marker(b"paint_image ok");
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, 0, 0, ctx.width, ctx.height);
    debug::marker(b"damage commit");
    true
}
