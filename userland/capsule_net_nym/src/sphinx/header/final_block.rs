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

use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{ENCRYPTED_ROUTING_INFO_SIZE, FINAL_HOP_FLAG, VERSION_LENGTH};
use crate::sphinx::final_routing::{
    add_padding, combine_with_filler, encrypt_final, FinalRoutingInformation,
};
use crate::sphinx::keys::ExpandedSharedSecret;
use crate::sphinx::node::Destination;

/// The last hop's routing block: destination and identifier, padded with
/// random bytes, sealed under the last hop's key, then completed by the filler.
pub fn build_final_block(
    last: &ExpandedSharedSecret,
    destination: &Destination,
    route_len: usize,
    version: [u8; VERSION_LENGTH],
    filler: &[u8],
) -> Result<[u8; ENCRYPTED_ROUTING_INFO_SIZE], CryptoError> {
    let info = FinalRoutingInformation {
        flag: FINAL_HOP_FLAG,
        version,
        destination: destination.address,
        identifier: destination.identifier,
    };
    let padded = add_padding(&info, route_len).ok_or(CryptoError::Kdf)?;
    let sealed = encrypt_final(&padded, &last.stream_cipher_key());
    combine_with_filler(&sealed, filler).ok_or(CryptoError::Kdf)
}
