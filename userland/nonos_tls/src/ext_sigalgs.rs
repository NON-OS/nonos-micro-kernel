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

use alloc::vec::Vec;

use super::constants::EXT_SIGNATURE_ALGORITHMS;

pub fn ext_sigalgs(out: &mut Vec<u8>) {
    // Only schemes the verifier actually handles are advertised, so the server
    // never picks one we would reject. rsa_pss_sha256/384, ecdsa P-256/P-384,
    // rsa_pkcs1_sha256. rsa_pss_sha512 (0x0806) is intentionally omitted: the
    // RSA verify path wires SHA-256 and SHA-384 only.
    let mut body = Vec::with_capacity(12);
    super::push::u16(&mut body, 10);
    super::push::u16(&mut body, 0x0804);
    super::push::u16(&mut body, 0x0805);
    super::push::u16(&mut body, 0x0403);
    super::push::u16(&mut body, 0x0503);
    super::push::u16(&mut body, 0x0401);
    super::push::ext(out, EXT_SIGNATURE_ALGORITHMS, &body);
}
