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

use egui::epaint::Primitive;
use egui::{pos2, vec2, Context, RawInput, Rect};

use super::manifest::{HEIGHT, WIDTH};

#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub shapes: u32,
    pub prims: u32,
    pub verts: u32,
    pub idxs: u32,
    pub textures: u32,
    pub atlas_w: u32,
    pub atlas_h: u32,
}

pub fn run(ctx: &Context, clicks: u32) -> Stats {
    let input = RawInput {
        screen_rect: Some(Rect::from_min_size(
            pos2(0.0, 0.0),
            vec2(WIDTH as f32, HEIGHT as f32),
        )),
        ..Default::default()
    };
    let output = ctx.run_ui(input, |ui| {
        ui.heading("egui on NONOS");
        ui.separator();
        ui.label(format!("clicks {clicks}"));
        let _ = ui.button("attested");
    });

    let mut stats = Stats {
        shapes: output.shapes.len() as u32,
        textures: output.textures_delta.set.len() as u32,
        ..Default::default()
    };
    for (_, delta) in &output.textures_delta.set {
        stats.atlas_w = stats.atlas_w.max(delta.image.width() as u32);
        stats.atlas_h = stats.atlas_h.max(delta.image.height() as u32);
    }

    let ppp = output.pixels_per_point;
    for clipped in ctx.tessellate(output.shapes, ppp) {
        if let Primitive::Mesh(mesh) = clipped.primitive {
            stats.prims += 1;
            stats.verts += mesh.vertices.len() as u32;
            stats.idxs += mesh.indices.len() as u32;
        }
    }
    stats
}
