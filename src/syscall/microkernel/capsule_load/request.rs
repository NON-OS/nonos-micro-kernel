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

// Wire layout the userland installer fills in and passes by pointer. The four
// artifact blobs are referenced by user pointer plus length; the installer reads
// them from the store and owns the memory for the duration of the call. The
// service name, endpoints, and target are taken from the signed manifest inside
// the kernel, so they are not part of this request. u64 fields are grouped first
// so the layout has no implicit padding and matches the libc mirror byte for
// byte.
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct CapsuleLoadRequest {
    pub elf_ptr: u64,
    pub cert_ptr: u64,
    pub manifest_ptr: u64,
    pub trailer_ptr: u64,
    pub requested_caps: u64,
    pub elf_len: u32,
    pub cert_len: u32,
    pub manifest_len: u32,
    pub trailer_len: u32,
}
