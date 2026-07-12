// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::driver::Driver;
use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_ALIVE_WAIT, OP_BEACON_PARSE,
    OP_DEVICE_INFO, OP_DMA_STATE, OP_FIRMWARE_INFO, OP_FIRMWARE_LOAD, OP_FIRMWARE_STAGE,
    OP_CCMP, OP_EAPOL_VERIFY, OP_HCMD_ISSUE, OP_HEALTHCHECK, OP_MGMT_BUILD, OP_RF_STATE,
    OP_RX_POLL, OP_WPA_PTK,
};
use crate::server::{handlers, respond};

const SERVICE_INBOX: u64 = 0;

pub fn run(mut driver: Driver) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&mut driver, sender_pid, req, body, &mut tx);
    }
}

fn dispatch(driver: &mut Driver, sender_pid: u32, req: crate::protocol::Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_DEVICE_INFO if body.is_empty() => handlers::device::handle(driver, sender_pid, &req, tx),
        OP_FIRMWARE_INFO if body.is_empty() => handlers::firmware::handle(driver, sender_pid, &req, tx),
        OP_RF_STATE if body.is_empty() => handlers::rf::handle(driver, sender_pid, &req, tx),
        OP_DMA_STATE if body.is_empty() => handlers::dma::handle(driver, sender_pid, &req, tx),
        OP_FIRMWARE_STAGE if body.is_empty() => {
            handlers::firmware_stage::handle(driver, sender_pid, &req, tx)
        }
        OP_FIRMWARE_LOAD if body.is_empty() => {
            handlers::firmware_load::handle(driver, sender_pid, &req, tx)
        }
        OP_ALIVE_WAIT if body.is_empty() => handlers::alive::handle(driver, sender_pid, &req, tx),
        OP_RX_POLL if body.is_empty() => handlers::rx::handle(driver, sender_pid, &req, tx),
        OP_MGMT_BUILD => handlers::mgmt::handle(sender_pid, &req, body, tx),
        OP_BEACON_PARSE => handlers::beacon::handle(sender_pid, &req, body, tx),
        OP_HCMD_ISSUE => handlers::hcmd::handle(driver, sender_pid, &req, body, tx),
        OP_WPA_PTK => handlers::wpa::handle(sender_pid, &req, body, tx),
        OP_EAPOL_VERIFY => handlers::eapol::handle(sender_pid, &req, body, tx),
        OP_CCMP => handlers::ccmp::handle(sender_pid, &req, body, tx),
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
