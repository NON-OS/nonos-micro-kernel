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

pub fn verify_p256(pk: &[u8; 65], sig: &[u8; 64], digest: &[u8; 32]) -> bool {
    let mut body = [0u8; 161];
    body[..65].copy_from_slice(pk);
    body[65..129].copy_from_slice(sig);
    body[129..].copy_from_slice(digest);
    super::crypto_status::crypto_status(18, &body)
}
