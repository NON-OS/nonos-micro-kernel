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
extern crate alloc;

use alloc::vec::Vec;

use super::super::error::WireError;
use super::super::pkt::{read_pkt, Pkt};
use super::ref_line::parse_ref;
use super::remote_ref::RemoteRef;

/// Parse `GET /info/refs?service=...`, for either service.
///
/// The body opens with a banner packet naming the service and a flush, then
/// one ref per packet. The first ref carries the server's capabilities after a
/// NUL, dropped here: this asks for nothing beyond the defaults, so claiming
/// none back is what keeps the request honest.
///
/// A repository with no refs still has to answer, so it sends one entry
/// named `capabilities^{}` purely to carry that list. It names no object and
/// is skipped, or a push would read it as a branch.
pub fn parse_advertisement(body: &[u8]) -> Result<Vec<RemoteRef>, WireError> {
    let mut at = match read_pkt(body)? {
        (Pkt::Data(d), used) if d.starts_with(b"# service=") => used,
        _ => return Err(WireError::NotSmartHttp),
    };

    let mut refs = Vec::new();
    let mut seen_any = false;
    while at < body.len() {
        let (pkt, used) = read_pkt(&body[at..])?;
        at += used;
        match pkt {
            Pkt::Data(line) => {
                seen_any = true;
                let parsed = parse_ref(line)?;
                if parsed.name != "capabilities^{}" {
                    refs.push(parsed);
                }
            }
            // The flush after the banner is skipped; the one after the refs
            // ends the advertisement.
            Pkt::Flush if !seen_any => continue,
            Pkt::Flush => break,
        }
    }
    Ok(refs)
}
