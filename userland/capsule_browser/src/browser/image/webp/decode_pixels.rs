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

use alloc::vec;
use alloc::vec::Vec;

use super::bitread::BitReader;
use super::color_cache::ColorCache;
use super::dist::map_distance;
use super::hgroup::HGroup;
use super::lz77::prefix_value;
use super::meta::Meta;

// Decode xsize by ysize ARGB pixels. Each pixel is a literal, a backward
// reference that copies an earlier run, or a color-cache index. The green
// tree carries the branch: below 256 is a literal green, then 24 length
// codes, then cache codes.
pub(super) fn decode_pixels(
    br: &mut BitReader,
    xsize: usize,
    ysize: usize,
    groups: &[HGroup],
    meta: Option<&Meta>,
    cache: &mut Option<ColorCache>,
) -> Option<Vec<u32>> {
    let total = xsize.checked_mul(ysize)?;
    let mut px = vec![0u32; total];
    let mut i = 0usize;
    while i < total {
        let g = pick(groups, meta, i % xsize, i / xsize)?;
        let sym = g.green.read(br) as u32;
        if br.eos {
            return None;
        }
        if sym < 256 {
            let red = g.red.read(br) as u32;
            let blue = g.blue.read(br) as u32;
            let alpha = g.alpha.read(br) as u32;
            let argb = (alpha << 24) | (red << 16) | (sym << 8) | blue;
            px[i] = argb;
            if let Some(c) = cache.as_mut() {
                c.insert(argb);
            }
            i += 1;
        } else if sym < 256 + 24 {
            let len = prefix_value(sym - 256, br) as usize;
            let dcode = prefix_value(g.dist.read(br) as u32, br);
            let dist = map_distance(dcode, xsize as i32) as usize;
            if dist == 0 || dist > i || i + len > total {
                return None;
            }
            for _ in 0..len {
                let v = px[i - dist];
                px[i] = v;
                if let Some(c) = cache.as_mut() {
                    c.insert(v);
                }
                i += 1;
            }
        } else {
            let v = cache.as_ref()?.lookup((sym - 256 - 24) as usize);
            px[i] = v;
            i += 1;
        }
    }
    Some(px)
}

fn pick<'a>(groups: &'a [HGroup], meta: Option<&Meta>, x: usize, y: usize) -> Option<&'a HGroup> {
    let idx = meta.map_or(0, |m| m.group_at(x, y));
    groups.get(idx)
}
