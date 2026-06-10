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
use uefi::proto::media::file::{File, RegularFile};
use uefi::table::boot::MemoryType;

use crate::log::logger::{log_error, log_info};

use super::types::{FileLoadError, FileResult, MAX_KERNEL_SIZE};

pub fn read_regular_file(
    file: &mut RegularFile,
    bs: &uefi::table::boot::BootServices,
) -> FileResult<Vec<u8>> {
    let mut info_buf = [0u8; 256];

    let file_size = match file.get_info::<uefi::proto::media::file::FileInfo>(&mut info_buf) {
        Ok(info) => info.file_size() as usize,
        Err(_) => {
            log_info("file", "Could not get file info, reading in chunks");
            return read_file_chunked(file, bs);
        }
    };

    if file_size > MAX_KERNEL_SIZE {
        log_error("file", "File exceeds maximum size");
        return Err(FileLoadError::TooLarge);
    }

    if file_size == 0 {
        log_info("file", "File is empty");
        return Ok(Vec::new());
    }

    let pages = (file_size + 4095) / 4096;
    let buffer_addr = bs
        .allocate_pages(uefi::table::boot::AllocateType::AnyPages, MemoryType::LOADER_DATA, pages)
        .map_err(|_| {
            log_error("file", "Failed to allocate buffer for file");
            FileLoadError::AllocationFailed
        })?;

    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer_addr as *mut u8, file_size) };

    let mut total_read = 0usize;
    while total_read < file_size {
        let remaining = &mut buffer[total_read..];
        match file.read(remaining) {
            Ok(0) => {
                log_error("file", "Unexpected EOF during file read");
                let _ = bs.free_pages(buffer_addr, pages);
                return Err(FileLoadError::ReadFailed);
            }
            Ok(n) => {
                total_read += n;
            }
            Err(_) => {
                let _ = bs.free_pages(buffer_addr, pages);
                log_error("file", "Failed to read file contents");
                return Err(FileLoadError::ReadFailed);
            }
        }
    }

    let mut result = Vec::with_capacity(file_size);
    result.extend_from_slice(buffer);

    let _ = bs.free_pages(buffer_addr, pages);

    log_info("file", "File loaded successfully");
    Ok(result)
}

pub fn read_file_chunked(
    file: &mut RegularFile,
    _bs: &uefi::table::boot::BootServices,
) -> FileResult<Vec<u8>> {
    let mut result = Vec::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut total_read = 0usize;

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                total_read += n;
                if total_read > MAX_KERNEL_SIZE {
                    log_error("file", "File exceeds maximum size during chunked read");
                    return Err(FileLoadError::TooLarge);
                }
                result.extend_from_slice(&buffer[..n]);
            }
            Err(_) => {
                log_error("file", "Read error during chunked read");
                return Err(FileLoadError::ReadFailed);
            }
        }
    }

    Ok(result)
}
