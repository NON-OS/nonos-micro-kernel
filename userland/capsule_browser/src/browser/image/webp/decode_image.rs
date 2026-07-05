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

use alloc::vec::Vec;

use super::bitread::BitReader;
use super::color_cache::ColorCache;
use super::decode_pixels::decode_pixels;
use super::hgroup::{read_hgroup, HGroup};
use super::meta::{subsample_size, Meta};

// Decode one entropy-coded ARGB image: the color cache config, an optional
// meta-Huffman image that partitions the picture into Huffman groups, the
// groups themselves, then the pixels. Sub-images (the meta image and the
// transform data) decode with recursion disabled.
pub(super) fn decode_image(
    br: &mut BitReader,
    xsize: usize,
    ysize: usize,
    allow_recursion: bool,
) -> Option<Vec<u32>> {
    let cache_bits = if br.read_bit() == 1 { br.read(4) } else { 0 };
    let meta = read_meta(br, xsize, ysize, allow_recursion)?;
    let count = meta.as_ref().map_or(1, Meta::group_count);
    let mut groups: Vec<HGroup> = Vec::with_capacity(count);
    for _ in 0..count {
        groups.push(read_hgroup(br, cache_bits)?);
    }
    let mut cache = (cache_bits > 0).then(|| ColorCache::new(cache_bits));
    decode_pixels(br, xsize, ysize, &groups, meta.as_ref(), &mut cache)
}

fn read_meta(
    br: &mut BitReader,
    xsize: usize,
    ysize: usize,
    allow_recursion: bool,
) -> Option<Option<Meta>> {
    if !allow_recursion || br.read_bit() == 0 {
        return Some(None);
    }
    let bits = br.read(3) + 2;
    let hx = subsample_size(xsize, bits);
    let hy = subsample_size(ysize, bits);
    let image = decode_image(br, hx, hy, false)?;
    Some(Some(Meta { image, bits, xsize: hx }))
}
