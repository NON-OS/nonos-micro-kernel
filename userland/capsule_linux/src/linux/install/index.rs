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

//! The distribution's package index, as this capsule needs it.

use alloc::string::String;
use alloc::vec::Vec;

pub struct Pkg {
    pub name: String,
    pub version: String,
}

pub struct Index {
    /// soname -> package
    pub libs: Vec<(String, Pkg)>,
    /// package name -> package
    pub names: Vec<(String, Pkg)>,
}

impl Index {
    pub fn parse(raw: &[u8]) -> Index {
        let text = String::from_utf8_lossy(raw);
        let (mut libs, mut names) = (Vec::new(), Vec::new());
        let (mut name, mut version) = (String::new(), String::new());
        for line in text.lines() {
            match line.as_bytes().first() {
                Some(b'P') => name = String::from(&line[2..]),
                Some(b'V') => {
                    version = String::from(&line[2..]);
                    names
                        .push((name.clone(), Pkg { name: name.clone(), version: version.clone() }));
                }
                Some(b'p') => {
                    for token in line[2..].split_whitespace() {
                        if let Some(so) = token.strip_prefix("so:") {
                            let so = so.split('=').next().unwrap_or(so);
                            let pkg = Pkg { name: name.clone(), version: version.clone() };
                            libs.push((String::from(so), pkg));
                        }
                    }
                }
                _ => {}
            }
        }
        Index { libs, names }
    }

    pub fn by_lib(&self, soname: &str) -> Option<&Pkg> {
        self.libs.iter().find(|(k, _)| k == soname).map(|(_, v)| v)
    }

    pub fn by_name(&self, name: &str) -> Option<&Pkg> {
        self.names.iter().find(|(k, _)| k == name).map(|(_, v)| v)
    }
}
