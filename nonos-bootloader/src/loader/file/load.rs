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

extern crate alloc;

use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::CStr16;

use crate::log::logger::{log_error, log_info};

use super::read::read_regular_file;
use super::types::{FileLoadError, FileResult};

pub fn load_file_from_esp(system_table: &SystemTable<Boot>, path: &CStr16) -> FileResult<Vec<u8>> {
    let bs = system_table.boot_services();

    let handles = bs.find_handles::<SimpleFileSystem>().map_err(|_| FileLoadError::NoFilesystem)?;

    if handles.is_empty() {
        log_error("file", "No filesystem handles found");
        return Err(FileLoadError::NoFilesystem);
    }

    for &handle in handles.iter() {
        if let Ok(mut fs) = bs.open_protocol_exclusive::<SimpleFileSystem>(handle) {
            if let Ok(mut root) = fs.open_volume() {
                match root.open(path, FileMode::Read, FileAttribute::empty()) {
                    Ok(file_handle) => {
                        let file_type = file_handle.into_type().map_err(|_| {
                            log_error("file", "Failed to determine file type");
                            FileLoadError::NotRegularFile
                        })?;

                        if let FileType::Regular(mut regular_file) = file_type {
                            return read_regular_file(&mut regular_file, bs);
                        } else {
                            log_error("file", "Path is not a regular file");
                            return Err(FileLoadError::NotRegularFile);
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    }

    log_error("file", "File not found on any volume");
    Err(FileLoadError::FileNotFound)
}

pub fn load_kernel_binary(system_table: &SystemTable<Boot>) -> FileResult<Vec<u8>> {
    log_info("file", "Searching for kernel binary...");

    log_info("file", "Trying \\EFI\\nonos\\kernel.bin");
    if let Ok(data) = load_file_from_esp(system_table, uefi::cstr16!("\\EFI\\nonos\\kernel.bin")) {
        log_info("file", "Kernel found and loaded");
        return Ok(data);
    }

    log_info("file", "Trying \\EFI\\nonos\\nonos_kernel");
    if let Ok(data) = load_file_from_esp(system_table, uefi::cstr16!("\\EFI\\nonos\\nonos_kernel"))
    {
        log_info("file", "Kernel found and loaded");
        return Ok(data);
    }

    log_info("file", "Trying \\nonos_kernel");
    if let Ok(data) = load_file_from_esp(system_table, uefi::cstr16!("\\nonos_kernel")) {
        log_info("file", "Kernel found and loaded");
        return Ok(data);
    }

    log_info("file", "Trying \\kernel.bin");
    if let Ok(data) = load_file_from_esp(system_table, uefi::cstr16!("\\kernel.bin")) {
        log_info("file", "Kernel found and loaded");
        return Ok(data);
    }

    log_error("file", "Kernel not found at any expected path");
    Err(FileLoadError::FileNotFound)
}

pub fn file_exists(system_table: &SystemTable<Boot>, path: &CStr16) -> bool {
    let bs = system_table.boot_services();

    if let Ok(handles) = bs.find_handles::<SimpleFileSystem>() {
        for &handle in handles.iter() {
            if let Ok(mut fs) = bs.open_protocol_exclusive::<SimpleFileSystem>(handle) {
                if let Ok(mut root) = fs.open_volume() {
                    if root.open(path, FileMode::Read, FileAttribute::empty()).is_ok() {
                        return true;
                    }
                }
            }
        }
    }
    false
}
