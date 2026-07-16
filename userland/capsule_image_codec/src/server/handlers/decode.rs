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
use nonos_libc::{mk_surface_attach, mk_surface_release, SurfaceDescriptor};
use nonos_toolkit::image::{bmp, gif, jpeg, lz4_raw, png, types::{DecodeError, ImageSize}};

use crate::protocol::{DECODE_LZ4_PREFIX_LEN, DECODE_REQ_LEN, DECODE_RESP_LEN, E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED, HDR_LEN, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG, OP_DECODE_LZ4_RAW, OP_DECODE_PNG, Request, STATUS_LEN};
use crate::server::{handlers::surface::register_argb_surface, respond};

const MAX_OUT_PIXELS: usize = 16_777_216;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < DECODE_REQ_LEN {
        return fail(sender_pid, req, E_BAD_LEN, tx);
    }
    let in_handle = u64::from_le_bytes([body[0], body[1], body[2], body[3], body[4], body[5], body[6], body[7]]);
    let byte_len = u64::from_le_bytes([body[8], body[9], body[10], body[11], body[12], body[13], body[14], body[15]]) as usize;
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(in_handle, &mut desc as *mut _);
    if va <= 0 || (byte_len as u64) > desc.byte_len {
        if va > 0 { let _ = mk_surface_release(in_handle); }
        return fail(sender_pid, req, E_BAD_LEN, tx);
    }
    let img = unsafe { core::slice::from_raw_parts(va as usize as *const u8, byte_len) };
    let result = decode_sized(req.op, img);
    let _ = mk_surface_release(in_handle);
    match result {
        Ok((pixels, size)) => finish(sender_pid, req, &pixels, size, tx),
        Err(e) => fail(sender_pid, req, e, tx),
    }
}

fn fail(sender_pid: u32, req: &Request, errno: i32, tx: &mut [u8]) { let _ = respond::status(sender_pid, req, errno, tx); }

fn peek_size(op: u16, img: &[u8]) -> Result<ImageSize, i32> {
    let r = match op {
        OP_DECODE_PNG => png::png_dimensions(img),
        OP_DECODE_BMP => bmp::bmp_dimensions(img),
        OP_DECODE_GIF => gif::gif_dimensions(img),
        OP_DECODE_JPEG => jpeg::parse_jpeg_header(img).map(|(s, _)| s),
        _ => return Err(E_INVAL),
    };
    r.map_err(map_decode_error)
}

fn decode_into(op: u16, img: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    match op {
        OP_DECODE_PNG => png::decoder::decode_png_argb8888(img, out),
        OP_DECODE_BMP => bmp::decode_bmp_argb8888(img, out),
        OP_DECODE_GIF => gif::decode_gif_argb8888(img, out),
        OP_DECODE_JPEG => jpeg::decode_jpeg_argb8888(img, out),
        OP_DECODE_LZ4_RAW => decode_lz4(img, out),
        _ => Err(DecodeError::Unsupported),
    }
}

fn decode_sized(op: u16, img: &[u8]) -> Result<(Vec<u32>, ImageSize), i32> {
    let size = if op == OP_DECODE_LZ4_RAW {
        if img.len() < DECODE_LZ4_PREFIX_LEN { return Err(E_BAD_LEN); }
        let w = u32::from_le_bytes([img[0], img[1], img[2], img[3]]);
        let h = u32::from_le_bytes([img[4], img[5], img[6], img[7]]);
        ImageSize::new(w, h).map_err(map_decode_error)?
    } else {
        peek_size(op, img)?
    };
    let count = size.pixel_count() as usize;
    if count == 0 || count > MAX_OUT_PIXELS {
        return Err(E_NOMEM);
    }
    let mut pixels = vec![0u32; count];
    let decoded = decode_into(op, img, &mut pixels).map_err(map_decode_error)?;
    Ok((pixels, decoded))
}

fn finish(sender_pid: u32, req: &Request, pixels: &[u32], size: ImageSize, tx: &mut [u8]) {
    let count = size.pixel_count() as usize;
    let (handle, stride, byte_len) = match register_argb_surface(&pixels[..count], size) {
        Ok(v) => v,
        Err(e) => return fail(sender_pid, req, e, tx),
    };
    let o = HDR_LEN + STATUS_LEN;
    tx[o..o + 8].copy_from_slice(&handle.to_le_bytes());
    tx[o + 8..o + 12].copy_from_slice(&size.width.to_le_bytes());
    tx[o + 12..o + 16].copy_from_slice(&size.height.to_le_bytes());
    tx[o + 16..o + 20].copy_from_slice(&stride.to_le_bytes());
    tx[o + 20..o + 24].copy_from_slice(&1u32.to_le_bytes());
    tx[o + 24..o + 32].copy_from_slice(&byte_len.to_le_bytes());
    let _ = respond::payload(sender_pid, req, DECODE_RESP_LEN, tx);
}

fn decode_lz4(body: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    if body.len() < DECODE_LZ4_PREFIX_LEN { return Err(DecodeError::Truncated); }
    let width = u32::from_le_bytes(body[0..4].try_into().map_err(|_| DecodeError::Truncated)?);
    let height = u32::from_le_bytes(body[4..8].try_into().map_err(|_| DecodeError::Truncated)?);
    lz4_raw::decode_lz4_raw_argb8888(width, height, &body[8..], out)
}

fn map_decode_error(err: DecodeError) -> i32 {
    match err { DecodeError::BadMagic => E_INVAL, DecodeError::Unsupported => E_UNSUPPORTED, DecodeError::BadDimensions => E_BAD_LEN, DecodeError::OutputTooSmall => E_BAD_LEN, DecodeError::Truncated => E_BAD_LEN }
}
