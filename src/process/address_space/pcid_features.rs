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

/// Whether a context switch can retire one address space's translations
/// instead of flushing the whole non-global set.
///
/// The question is the same on both architectures and the answer comes from
/// the paging boundary, which asks CPUID for PCID and `invpcid` on x86_64 and
/// answers for ASID on aarch64. What the caller does with a `false` is
/// unchanged: flush everything and carry on, slower.
pub(super) fn supports_pcid_invalidation() -> bool {
    crate::arch::paging::supports_tagged_invalidation()
}
