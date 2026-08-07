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

pub const MAGIC: &[u8; 4] = b"NOS1";

pub(crate) const HEADER_LEN: usize = 8;
pub(crate) const ENTRY_LEN: usize = 16;

#[derive(Clone, Copy)]
pub struct Sections<'a> {
    pub manifest: &'a [u8],
    pub elf: &'a [u8],
    pub id_cert: &'a [u8],
    pub zk_trailer: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PkgErr {
    Short,
    BadMagic,
    BadVersion,
    BadCount,
    DupKind,
    OutOfOrder,
    BadExtent,
    MissingSection,
    BadTrailer,
}
