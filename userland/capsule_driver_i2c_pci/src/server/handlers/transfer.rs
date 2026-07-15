use crate::driver::Driver;
use crate::protocol::{Request, E_BUSY, E_INVAL, E_NACK, E_OK, E_TIMEOUT, TRANSFER_READ_MAX};
use crate::server::respond;
use crate::transaction::{transfer, TransferError, TransferRequest};

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    let Some((addr, flags, write, read_len)) = parse(body) else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    let tr = TransferRequest { addr, flags, write, read_len };
    match transfer(driver, tr) {
        Ok(result) => {
            reply_ok(sender_pid, req, &result.read[..result.read_len], result.abort_source, out)
        }
        Err(e) => {
            let _ = respond::send(sender_pid, req, errno(e), &[], out);
        }
    }
}

fn parse(body: &[u8]) -> Option<(u8, u16, &[u8], usize)> {
    if body.len() < 8 {
        return None;
    }
    let addr = body[0];
    let write_len = u16::from_le_bytes(body[2..4].try_into().ok()?) as usize;
    let read_len = u16::from_le_bytes(body[4..6].try_into().ok()?) as usize;
    let flags = u16::from_le_bytes(body[6..8].try_into().ok()?);
    if body.len() != 8 + write_len {
        return None;
    }
    Some((addr, flags, &body[8..], read_len))
}

fn reply_ok(sender_pid: u32, req: &Request, read: &[u8], abort: u32, out: &mut [u8]) {
    let mut body = [0u8; 8 + TRANSFER_READ_MAX];
    body[0..2].copy_from_slice(&(read.len() as u16).to_le_bytes());
    body[4..8].copy_from_slice(&abort.to_le_bytes());
    body[8..8 + read.len()].copy_from_slice(read);
    let _ = respond::send(sender_pid, req, E_OK, &body[..8 + read.len()], out);
}

fn errno(err: TransferError) -> i32 {
    match err {
        TransferError::Busy => E_BUSY,
        TransferError::Timeout => E_TIMEOUT,
        TransferError::Nack => E_NACK,
        TransferError::Invalid => E_INVAL,
    }
}
