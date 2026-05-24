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

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::{PRODUCT_NAME, PRODUCT_TAGLINE, PRODUCT_VERSION};

pub fn run(out: &mut [u8], req: &Request) -> usize {
    let mut written = 0usize;
    let dst = HDR_LEN + STATUS_LEN;
    written += write_field(&mut out[dst + written..], PRODUCT_NAME);
    written += write_field(&mut out[dst + written..], PRODUCT_TAGLINE);
    written += write_field(&mut out[dst + written..], PRODUCT_VERSION);
    if dst + written > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    respond::with_payload(out, req, 0, written)
}

fn write_field(out: &mut [u8], bytes: &[u8]) -> usize {
    if out.len() < 4 + bytes.len() {
        return 0;
    }
    out[0..4].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
    out[4..4 + bytes.len()].copy_from_slice(bytes);
    4 + bytes.len()
}
