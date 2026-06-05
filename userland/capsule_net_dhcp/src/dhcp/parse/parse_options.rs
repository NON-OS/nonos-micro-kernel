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

use super::error::ParseError;
use crate::dhcp::constants::*;
use crate::dhcp::message::Message;

pub(super) fn parse_options(bytes: &[u8], message: &mut Message) -> Result<(), ParseError> {
    let mut i = HEADER_LEN;
    while i < bytes.len() {
        let tag = bytes[i];
        i += 1;
        if tag == OPT_END {
            break;
        }
        if tag == OPT_PAD {
            continue;
        }
        if i >= bytes.len() {
            return Err(ParseError::BadOption);
        }
        let len = bytes[i] as usize;
        i += 1;
        if i + len > bytes.len() {
            return Err(ParseError::BadOption);
        }
        match tag {
            OPT_MESSAGE_TYPE if len == 1 => message.message_type = bytes[i],
            OPT_SUBNET_MASK if len == 4 => message.subnet_mask.copy_from_slice(&bytes[i..i + 4]),
            OPT_ROUTER if len >= 4 => message.router.copy_from_slice(&bytes[i..i + 4]),
            OPT_DNS if len >= 4 => message.dns.copy_from_slice(&bytes[i..i + 4]),
            OPT_LEASE_TIME if len == 4 => {
                message.lease_seconds =
                    u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
            }
            OPT_SERVER_IDENTIFIER if len == 4 => {
                message.server_id.copy_from_slice(&bytes[i..i + 4]);
            }
            _ => {}
        }
        i += len;
    }
    Ok(())
}
