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
use super::parse_fixed::parse_fixed;
use super::parse_options::parse_options;
use crate::dhcp::constants::{FIELD_COOKIE, HEADER_LEN, MAGIC_COOKIE};
use crate::dhcp::message::Message;

pub fn parse(bytes: &[u8]) -> Result<Message, ParseError> {
    if bytes.len() < HEADER_LEN {
        return Err(ParseError::TooShort);
    }
    if bytes[FIELD_COOKIE..FIELD_COOKIE + 4] != MAGIC_COOKIE {
        return Err(ParseError::BadCookie);
    }
    let mut message = parse_fixed(bytes);
    parse_options(bytes, &mut message)?;
    Ok(message)
}
