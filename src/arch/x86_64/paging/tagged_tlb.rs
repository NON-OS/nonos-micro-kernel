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

/// PCID in leaf 1, and the `invpcid` instruction that makes it useful in
/// leaf 7. Both are needed: a CPU that tags entries but cannot invalidate by
/// tag leaves no way to retire one address space, so the pair is reported as
/// a single answer rather than two.
pub fn supports_tagged_invalidation() -> bool {
    // Leaves 1 and 7 are architectural on every CPU this kernel boots on, both
    // below the maximum leaf any x86_64 part reports, so neither read needs a
    // guard.
    let base = core::arch::x86_64::__cpuid(1);
    let ext = core::arch::x86_64::__cpuid_count(7, 0);
    let (pcid, invpcid) = (base.ecx & (1 << 17) != 0, ext.ebx & (1 << 10) != 0);

    if !pcid {
        crate::log::log_warning!("[ADDR_SPACE] PCID not supported by CPU");
        return false;
    }
    if !invpcid {
        crate::log::log_warning!("[ADDR_SPACE] INVPCID not supported by CPU");
        return false;
    }
    true
}
