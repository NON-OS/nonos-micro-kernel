// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Hostname and domainname management

use crate::sys::sync::IrqMutex;

struct HostnameState {
    hostname: [u8; 64],
    hostname_len: usize,
    domainname: [u8; 64],
    domainname_len: usize,
}

static STATE: IrqMutex<HostnameState> = IrqMutex::new(HostnameState {
    hostname: [0; 64],
    hostname_len: 0,
    domainname: [0; 64],
    domainname_len: 0,
});

pub fn init() {
    let mut state = STATE.lock();
    state.hostname[..5].copy_from_slice(b"nonos");
    state.hostname_len = 5;
}

pub fn get() -> alloc::string::String {
    let state = STATE.lock();
    if state.hostname_len == 0 {
        alloc::string::String::from("nonos")
    } else {
        alloc::string::String::from_utf8_lossy(&state.hostname[..state.hostname_len]).into_owned()
    }
}

pub fn set(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("Hostname cannot be empty");
    }
    if name.len() > 63 {
        return Err("Hostname too long");
    }
    if !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'.') {
        return Err("Invalid hostname characters");
    }

    let mut state = STATE.lock();
    state.hostname[..name.len()].copy_from_slice(name.as_bytes());
    state.hostname_len = name.len();
    Ok(())
}

pub fn get_domain() -> alloc::string::String {
    let state = STATE.lock();
    if state.domainname_len == 0 {
        alloc::string::String::new()
    } else {
        alloc::string::String::from_utf8_lossy(&state.domainname[..state.domainname_len])
            .into_owned()
    }
}

pub fn set_domain(name: &str) -> Result<(), &'static str> {
    if name.len() > 63 {
        return Err("Domainname too long");
    }

    let mut state = STATE.lock();
    if name.is_empty() {
        state.domainname_len = 0;
    } else {
        state.domainname[..name.len()].copy_from_slice(name.as_bytes());
        state.domainname_len = name.len();
    }
    Ok(())
}
