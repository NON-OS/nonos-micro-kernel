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

use crate::image::jpeg::dht::{parse_dht, HuffmanTable, MAX_HT};
use crate::image::jpeg::dqt::{parse_dqt, QuantTable, MAX_QT};
use crate::image::jpeg::marker::{
    is_app, is_rst, is_sof_unsupported, read_marker, read_segment_len,
    M_COM, M_DHT, M_DQT, M_DRI, M_EOI, M_SOF0, M_SOI, M_SOS,
};
use crate::image::jpeg::mcu::{walk_scan, ScanContext};
use crate::image::jpeg::sof0::{parse_sof0, FrameHeader};
use crate::image::jpeg::sos::parse_sos;
use crate::image::types::{DecodeError, ImageSize};

pub fn parse_jpeg_header(input: &[u8]) -> Result<(ImageSize, u8), DecodeError> {
    if input.len() < 2 || input[0] != 0xFF || input[1] != M_SOI {
        return Err(DecodeError::BadMagic);
    }
    let mut pos = 2usize;
    loop {
        let marker = read_marker(input, &mut pos)?;
        match marker {
            M_SOI => continue,
            M_EOI => return Err(DecodeError::Unsupported),
            M_SOF0 => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let seg = &input[pos..pos + seg_len];
                let frame = parse_sof0(seg)?;
                let size = ImageSize::new(frame.width as u32, frame.height as u32)?;
                return Ok((size, frame.num_comps));
            }
            m if is_sof_unsupported(m) => return Err(DecodeError::Unsupported),
            m if is_rst(m) || m == 0x01 => continue,
            _ => {
                let seg_len = read_segment_len(input, &mut pos)?;
                pos += seg_len;
            }
        }
    }
}

pub fn decode_jpeg_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    if input.len() < 2 || input[0] != 0xFF || input[1] != M_SOI {
        return Err(DecodeError::BadMagic);
    }
    let mut pos = 2usize;
    let mut frame_opt: Option<FrameHeader> = None;
    let mut dqt: [QuantTable; MAX_QT] = [QuantTable::new(); MAX_QT];
    let mut dc_tables: [HuffmanTable; MAX_HT] = [
        HuffmanTable::new(), HuffmanTable::new(), HuffmanTable::new(), HuffmanTable::new(),
    ];
    let mut ac_tables: [HuffmanTable; MAX_HT] = [
        HuffmanTable::new(), HuffmanTable::new(), HuffmanTable::new(), HuffmanTable::new(),
    ];
    let mut restart_interval: u32 = 0;
    loop {
        let marker = read_marker(input, &mut pos)?;
        match marker {
            M_SOI => continue,
            M_EOI => return Err(DecodeError::Truncated),
            M_SOF0 => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let seg = &input[pos..pos + seg_len];
                let frame = parse_sof0(seg)?;
                let size = ImageSize::new(frame.width as u32, frame.height as u32)?;
                if (size.pixel_count() as usize) > out.len() {
                    return Err(DecodeError::OutputTooSmall);
                }
                frame_opt = Some(frame);
                pos += seg_len;
            }
            m if is_sof_unsupported(m) => return Err(DecodeError::Unsupported),
            M_DQT => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let seg = &input[pos..pos + seg_len];
                parse_dqt(seg, &mut dqt)?;
                pos += seg_len;
            }
            M_DHT => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let seg = &input[pos..pos + seg_len];
                parse_dht(seg, &mut dc_tables, &mut ac_tables)?;
                pos += seg_len;
            }
            M_DRI => {
                let seg_len = read_segment_len(input, &mut pos)?;
                if seg_len < 2 {
                    return Err(DecodeError::Truncated);
                }
                restart_interval =
                    u16::from_be_bytes([input[pos], input[pos + 1]]) as u32;
                pos += seg_len;
            }
            M_SOS => {
                let seg_len = read_segment_len(input, &mut pos)?;
                let seg = &input[pos..pos + seg_len];
                let frame = frame_opt.as_ref().ok_or(DecodeError::Unsupported)?;
                let scan = parse_sos(seg, frame)?;
                pos += seg_len;
                let mut i = 0usize;
                while i < frame.num_comps as usize {
                    let tq = frame.comps[i].tq as usize;
                    if tq >= MAX_QT || !dqt[tq].present {
                        return Err(DecodeError::Unsupported);
                    }
                    i += 1;
                }
                let ctx = ScanContext {
                    frame,
                    scan: &scan,
                    dc_tables: &dc_tables,
                    ac_tables: &ac_tables,
                    qt: &dqt,
                    restart_interval,
                };
                walk_scan(&ctx, input, pos, out)?;
                let size = ImageSize::new(frame.width as u32, frame.height as u32)?;
                return Ok(size);
            }
            M_COM => {
                let seg_len = read_segment_len(input, &mut pos)?;
                pos += seg_len;
            }
            m if is_app(m) => {
                let seg_len = read_segment_len(input, &mut pos)?;
                pos += seg_len;
            }
            m if is_rst(m) || m == 0x01 => continue,
            _ => {
                let seg_len = read_segment_len(input, &mut pos)?;
                pos += seg_len;
            }
        }
    }
}
