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
pub const VQ_REGION_SIZE: u64 = 16384;
pub const VQ_MAX_SIZE: u16 = 256;
pub const VQ_DESC_OFFSET: usize = 0;
pub const VQ_AVAIL_OFFSET: usize = 4096;
pub const VQ_USED_OFFSET: usize = 8192;
pub const VQ_STAGING_OFFSET: usize = 12288;
pub const VQ_STAGING_LEN: usize = 4096;
pub const VRING_DESC_F_NEXT: u16 = 1;
pub const VRING_DESC_F_WRITE: u16 = 2;
pub const CTRLQ_INDEX: u16 = 0;
