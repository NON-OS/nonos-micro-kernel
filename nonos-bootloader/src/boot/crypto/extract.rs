// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::{log_hash, log_ok, BootCryptoState};
use crate::image_format::{has_production_footer, parse_image_footer};

pub fn extract_signature_for_display(data: &[u8], state: &mut BootCryptoState, gop: bool) {
    if !has_production_footer(data) {
        return;
    }
    if let Ok(parsed) = parse_image_footer(data) {
        let sig = parsed.signature_bytes;
        if sig.len() >= 64 {
            state.signature_r.copy_from_slice(&sig[0..32]);
            state.signature_s.copy_from_slice(&sig[32..64]);
            if gop {
                log_ok(b"Ed25519 signature extracted");
                log_hash(b"sig.R  ", &state.signature_r);
                log_hash(b"sig.S  ", &state.signature_s);
            }
        }
    }
}
