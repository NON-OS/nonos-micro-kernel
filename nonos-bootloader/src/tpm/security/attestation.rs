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

use crate::tpm::core::{TmpDevice, TmpError, TmpResult};
use crate::tpm::types::Quote;

pub fn create_attestation(
    device: &mut TmpDevice,
    nonce: &[u8; 32],
    pcr_mask: u32,
) -> TmpResult<Quote> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    let mut cmd = [0u8; 64];
    cmd[0..10].copy_from_slice(&[0x80, 0x02, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x01, 0x58]);
    cmd[10..14].copy_from_slice(&0x40000007u32.to_be_bytes());
    cmd[14..18].copy_from_slice(&pcr_mask.to_be_bytes());
    cmd[18..22].copy_from_slice(&32u32.to_be_bytes());
    cmd[22..54].copy_from_slice(nonce);
    cmd[54..64].copy_from_slice(&[0x00, 0x10, 0x00, 0x20, 0x00, 0x0B, 0x03, 0x00, 0x00, 0x00]);
    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < 64 {
        return Err(TmpError::InvalidResponse);
    }
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&response[16..48]);
    let mut sig = [0u8; 256];
    sig[..128].copy_from_slice(&response[48..176]);
    Ok(Quote { magic: 0xff544347u32, pcr_mask, pcr_digest: digest, signature: sig })
}

pub fn verify_quote(quote: &Quote, expected_nonce: &[u8; 32]) -> TmpResult<bool> {
    if quote.magic != 0xff544347u32 {
        return Ok(false);
    }
    if quote.pcr_digest.len() != 32 {
        return Ok(false);
    }
    for (q_byte, n_byte) in quote.pcr_digest.iter().zip(expected_nonce.iter()) {
        if q_byte != n_byte {
            return Ok(false);
        }
    }
    Ok(true)
}
