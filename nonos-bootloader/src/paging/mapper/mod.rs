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

// The index helpers and ensure_* frame walkers are internal to the
// mapper module; only the two public mapping entry points
// (`map_4k_run`, `map_huge_1g_run`) escape via re-export.

mod ensure_pd;
mod ensure_pdpt;
mod ensure_pt;
mod map_4k_run;
mod map_huge_1g_run;
mod map_huge_2m_run;
mod pd_index;
mod pdpt_index;
mod pml4_index;
mod pt_index;

pub use map_4k_run::map_4k_run;
pub use map_huge_1g_run::map_huge_1g_run;
pub use map_huge_2m_run::map_huge_2m_run;
