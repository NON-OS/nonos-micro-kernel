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

//! BIP-32/44 derivation paths.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::extended_key::HARDENED_OFFSET;
use crate::crypto::{CryptoError, CryptoResult};

pub const BIP44_PURPOSE: u32 = 44;
pub const BIP44_ETH_COIN: u32 = 60;
pub const BIP44_ETH_PATH: &str = "m/44'/60'/0'/0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathComponent {
    index: u32,
    hardened: bool,
}

impl PathComponent {
    pub const fn new(index: u32, hardened: bool) -> Self {
        Self { index, hardened }
    }

    pub const fn hardened(index: u32) -> Self {
        Self { index, hardened: true }
    }

    pub const fn normal(index: u32) -> Self {
        Self { index, hardened: false }
    }

    pub const fn index(&self) -> u32 {
        self.index
    }

    pub const fn is_hardened(&self) -> bool {
        self.hardened
    }

    pub const fn to_index(&self) -> u32 {
        if self.hardened {
            self.index | HARDENED_OFFSET
        } else {
            self.index
        }
    }
}

#[derive(Debug, Clone)]
pub struct DerivationPath {
    components: Vec<PathComponent>,
}

impl DerivationPath {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    pub fn from_components(components: Vec<PathComponent>) -> Self {
        Self { components }
    }

    pub fn bip44_eth(account: u32, address_index: u32) -> Self {
        Self {
            components: alloc::vec![
                PathComponent::hardened(BIP44_PURPOSE),
                PathComponent::hardened(BIP44_ETH_COIN),
                PathComponent::hardened(account),
                PathComponent::normal(0),
                PathComponent::normal(address_index),
            ],
        }
    }

    pub fn bip44_eth_account(account: u32) -> Self {
        Self {
            components: alloc::vec![
                PathComponent::hardened(BIP44_PURPOSE),
                PathComponent::hardened(BIP44_ETH_COIN),
                PathComponent::hardened(account),
            ],
        }
    }

    pub fn parse(path: &str) -> CryptoResult<Self> {
        let path = path.trim();

        if path.is_empty() || path == "m" {
            return Ok(Self::new());
        }

        let path = if path.starts_with("m/") {
            &path[2..]
        } else if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };

        let mut components = Vec::new();

        for part in path.split('/') {
            if part.is_empty() {
                continue;
            }

            let (index_str, hardened) =
                if part.ends_with('\'') || part.ends_with('H') || part.ends_with('h') {
                    (&part[..part.len() - 1], true)
                } else {
                    (part, false)
                };

            let index: u32 = index_str.parse().map_err(|_| CryptoError::InvalidInput)?;

            if index >= HARDENED_OFFSET {
                return Err(CryptoError::InvalidInput);
            }

            components.push(PathComponent::new(index, hardened));
        }

        Ok(Self { components })
    }

    pub fn components(&self) -> &[PathComponent] {
        &self.components
    }

    pub fn depth(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn push(&mut self, component: PathComponent) {
        self.components.push(component);
    }

    pub fn child(&self, component: PathComponent) -> Self {
        let mut new_path = self.clone();
        new_path.push(component);
        new_path
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("m");
        for c in &self.components {
            s.push('/');
            s.push_str(&alloc::format!("{}", c.index));
            if c.hardened {
                s.push('\'');
            }
        }
        s
    }
}

impl Default for DerivationPath {
    fn default() -> Self {
        Self::new()
    }
}
