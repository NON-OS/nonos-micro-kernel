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

//! A fresh guest, before anything has been put in it.

use alloc::vec::Vec;

use super::fd::Fd;
use super::handle::Guest;
use super::layout::{BRK_BASE, MMAP_BASE};

impl Guest {
    pub fn new(pid: u32) -> Self {
        Guest {
            pid,
            brk: BRK_BASE,
            mmap_next: MMAP_BASE,
            fds: Fd::standard(),
            regions: Vec::new(),
            pipes: Vec::new(),
            children: Vec::new(),
            threads: Vec::new(),
            waits: Vec::new(),
            handlers: [false; 64],
            display: Default::default(),
            objects: Default::default(),
            scene: Default::default(),
            cwd: alloc::vec![b'/'],
            automap: Vec::new(),
            fs_base: 0,
            exited: None,
            /*
             * The personality hosts it, and a fresh guest leads its own group
             * and session until something says otherwise.
             */
            parent: nonos_libc::mk_getpid(),
            pgid: pid,
            sid: pid,
            umask: crate::linux::call::DEFAULT_UMASK,
        }
    }
}
