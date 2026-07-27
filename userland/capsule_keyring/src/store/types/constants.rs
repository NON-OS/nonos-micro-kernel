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
pub const MAX_KEY_SIZE: usize = 256;
pub const MAX_KEYS: usize = 128;
// Per-owner cap: one capsule cannot fill the whole store and starve the other
// capsules that share the keyring. Generous for real wallet use, far below the
// global limit.
pub const MAX_KEYS_PER_OWNER: usize = 16;
