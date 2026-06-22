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

pub struct TransparentProof<'a> {
    pub commitment: &'a [u8; 32],
    pub nonce_point: &'a [u8; 32],
    pub z_x: &'a [u8; 32],
    pub z_r: &'a [u8; 32],
    pub depth: u8,
    pub siblings: &'a [u8],
    pub dirs: &'a [u8],
}
