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

pub(super) mod ops;
mod pack;
mod precomp;
mod scalarmult;
pub(super) mod types;

pub(crate) use ops::{ge_add, ge_p1p1_to_p3, ge_to_cached};
pub(crate) use pack::{ge_pack, ge_unpack};
pub(crate) use precomp::ensure_precomp;
pub(crate) use scalarmult::ge_scalarmult_vartime as scalarmult_vartime;
pub(crate) use scalarmult::{ge_has_large_order, ge_scalarmult_base_ct};
