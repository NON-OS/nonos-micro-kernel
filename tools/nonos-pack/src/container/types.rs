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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Manifest = 1,
    Elf = 2,
    IdCert = 3,
    ZkTrailer = 4,
}

impl SectionKind {
    pub fn from_u16(v: u16) -> Option<Self> {
        match v {
            1 => Some(SectionKind::Manifest),
            2 => Some(SectionKind::Elf),
            3 => Some(SectionKind::IdCert),
            4 => Some(SectionKind::ZkTrailer),
            _ => None,
        }
    }
}

pub struct Section {
    pub kind: SectionKind,
    pub bytes: Vec<u8>,
}

pub struct Container {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub enum PackErr {
    BadMagic,
    BadVersion,
    Truncated,
    OutOfOrder,
    Duplicate,
    UnknownKind,
    NoTrailer,
    NonCanonicalTrailer,
    SignatureTooLong,
    MissingSection,
    BadCert(String),
    MissingSignature(&'static str),
    MissingPublisherKey(&'static str),
    BadSignature(&'static str),
    Crypto(String),
}

pub const MAGIC: &[u8; 4] = b"NOS1";
pub const HEADER_LEN: usize = 8;
pub const ENTRY_LEN: usize = 16;
