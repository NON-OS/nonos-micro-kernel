use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::driver::Driver;
use crate::protocol::{
    parse, E_BAD_OP, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX, OP_ACPI_HID, OP_CONTROLLER_INFO,
    OP_HEALTHCHECK, OP_PROBE, OP_REGISTER_SNAPSHOT, OP_TIMING_INFO, OP_TRANSFER,
};
use crate::server::{handlers, respond};

const SERVICE_INBOX: u64 = 0;

pub fn run(driver: Driver) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let Some((req, body)) = parse(&rx[..n as usize]) else { continue };
        dispatch(&driver, sender_pid, req, body, &mut tx);
    }
}

fn dispatch(
    driver: &Driver,
    sender_pid: u32,
    req: crate::protocol::Request,
    body: &[u8],
    tx: &mut [u8],
) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender_pid, &req, tx),
        OP_CONTROLLER_INFO if body.is_empty() => {
            handlers::controller::handle(driver, sender_pid, &req, tx)
        }
        OP_REGISTER_SNAPSHOT if body.is_empty() => {
            handlers::snapshot::handle(driver, sender_pid, &req, tx)
        }
        OP_TIMING_INFO if body.is_empty() => handlers::timing::handle(driver, sender_pid, &req, tx),
        OP_TRANSFER => handlers::transfer::handle(driver, sender_pid, &req, body, tx),
        OP_PROBE => handlers::probe::handle(driver, sender_pid, &req, body, tx),
        OP_ACPI_HID if body.is_empty() => handlers::acpi_hid::handle(driver, sender_pid, &req, tx),
        _ if body.is_empty() => {
            let _ = respond::send(sender_pid, &req, E_BAD_OP, &[], tx);
        }
        _ => {
            let _ = respond::send(sender_pid, &req, E_INVAL, &[], tx);
        }
    }
}
