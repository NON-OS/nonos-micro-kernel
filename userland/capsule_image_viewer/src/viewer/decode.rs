extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_call, mk_surface_attach, mk_surface_release, SurfaceDescriptor};
use nonos_app_skeleton::wire::build_request;
use nonos_app_skeleton::discover::lookup_service;

const NIMG: u32 = 0x474D_494E;
const OP_PNG: u16 = 0x0002;
const OP_BMP: u16 = 0x0003;
const OP_JPEG: u16 = 0x0005;

pub struct Decoded { pub px: Vec<u32>, pub w: u32, pub h: u32 }

fn op_for(name: &[u8]) -> Result<u16, &'static str> {
    let dot = name.iter().rposition(|&b| b == b'.').ok_or("no ext")? + 1;
    let e = &name[dot..];
    let low = |x: &[u8], y: &[u8]| x.len() == y.len()
        && x.iter().zip(y).all(|(a, b)| a.to_ascii_lowercase() == *b);
    if low(e, b"png") { Ok(OP_PNG) }
    else if low(e, b"bmp") { Ok(OP_BMP) }
    else if low(e, b"jpg") || low(e, b"jpeg") { Ok(OP_JPEG) }
    else { Err("unsupported ext") }
}

pub fn decode(bytes: &[u8], name: &[u8]) -> Result<Decoded, &'static str> {
    let op = op_for(name)?;
    let svc = lookup_service(b"image_codec").ok_or("no codec")?;
    let tx = build_request(NIMG, op, 1, bytes);
    let mut rx = [0u8; 56];
    let rc = mk_ipc_call(svc.port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < 48 { return Err("codec call failed"); }
    if i32::from_le_bytes(rx[20..24].try_into().unwrap()) != 0 { return Err("decode error"); }
    let handle = u64::from_le_bytes(rx[24..32].try_into().unwrap());
    let w = u32::from_le_bytes(rx[32..36].try_into().unwrap());
    let h = u32::from_le_bytes(rx[36..40].try_into().unwrap());
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut desc as *mut _);
    if va <= 0 { return Err("attach failed"); }
    if w != desc.width || h != desc.height {
        let _ = mk_surface_release(handle);
        return Err("codec dim mismatch");
    }
    if (desc.width as u64) * (desc.height as u64) * 4 > desc.byte_len {
        let _ = mk_surface_release(handle);
        return Err("surface too small");
    }
    if (desc.stride as u64) < (desc.width as u64) * 4
        || (desc.stride as u64) * (desc.height as u64) > desc.byte_len {
        let _ = mk_surface_release(handle);
        return Err("bad surface geometry");
    }
    let px = copy_surface(va as usize, desc.stride, desc.width, desc.height);
    let _ = mk_surface_release(handle);
    Ok(Decoded { px, w: desc.width, h: desc.height })
}

fn copy_surface(base_va: usize, stride_bytes: u32, w: u32, h: u32) -> Vec<u32> {
    let mut out = vec![0u32; w as usize * h as usize];
    let mut y = 0usize;
    while y < h as usize {
        let row = (base_va + y * stride_bytes as usize) as *const u32;
        let mut x = 0usize;
        while x < w as usize {
            out[y * w as usize + x] = unsafe { core::ptr::read_volatile(row.add(x)) };
            x += 1;
        }
        y += 1;
    }
    out
}
