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

use super::base58::decode32;
use super::exit::Exit;

/// Parse a Nym address: identity.encryption@gateway, each base58.
///
/// The three keys do different jobs and are not interchangeable. The identity
/// names the requester, the encryption key seals what only it should read, and
/// the gateway is where the packet is handed off. Mixing them up produces an
/// address that parses and reaches nobody.
pub fn parse_address(text: &[u8]) -> Option<Exit> {
    let dot = text.iter().position(|c| *c == b'.')?;
    let at = text.iter().position(|c| *c == b'@')?;
    if dot >= at {
        return None;
    }
    Some(Exit {
        identity: decode32(&text[..dot])?,
        encryption: decode32(&text[dot + 1..at])?,
        gateway: decode32(&text[at + 1..])?,
    })
}
