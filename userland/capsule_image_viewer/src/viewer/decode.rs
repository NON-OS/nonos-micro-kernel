extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use nonos_app_skeleton::discover::lookup_service;
use nonos_app_skeleton::wire::build_request;
use nonos_libc::{
    mk_ipc_call, mk_mmap, mk_munmap, mk_surface_attach, mk_surface_register, mk_surface_release,
    mk_surface_share, SurfaceDescriptor, SURFACE_FORMAT_ARGB8888,
};

const NIMG: u32 = 0x474D_494E;
const OP_PNG: u16 = 0x0002;
const OP_BMP: u16 = 0x0003;
const OP_JPEG: u16 = 0x0005;
const OP_GIF: u16 = 0x0006;

pub struct Decoded {
    pub px: Vec<u32>,
    pub w: u32,
    pub h: u32,
}

fn op_for(name: &[u8]) -> Result<u16, &'static str> {
    let dot = name.iter().rposition(|&b| b == b'.').ok_or("no ext")? + 1;
    let e = &name[dot..];
    let low = |x: &[u8], y: &[u8]| {
        x.len() == y.len() && x.iter().zip(y).all(|(a, b)| a.to_ascii_lowercase() == *b)
    };
    if low(e, b"png") {
        Ok(OP_PNG)
    } else if low(e, b"bmp") {
        Ok(OP_BMP)
    } else if low(e, b"jpg") || low(e, b"jpeg") {
        Ok(OP_JPEG)
    } else if low(e, b"gif") {
        Ok(OP_GIF)
    } else {
        Err("unsupported ext")
    }
}

pub fn decode(bytes: &[u8], name: &[u8]) -> Result<Decoded, &'static str> {
    let op = op_for(name)?;
    let svc = lookup_service(b"image_codec").ok_or("no codec")?;
    let (in_handle, in_len, in_base, in_pad) = share_input(bytes)?;
    let mut payload = [0u8; 16];
    payload[0..8].copy_from_slice(&in_handle.to_le_bytes());
    payload[8..16].copy_from_slice(&in_len.to_le_bytes());
    let tx = build_request(NIMG, op, 1, &payload);
    let mut rx = [0u8; 56];
    let rc = mk_ipc_call(svc.port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    let _ = mk_surface_release(in_handle);
    let _ = mk_munmap(in_base, in_pad);
    if rc < 48 {
        return Err("codec call failed");
    }
    if i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) != 0 {
        return Err("decode error");
    }
    let handle =
        u64::from_le_bytes([rx[24], rx[25], rx[26], rx[27], rx[28], rx[29], rx[30], rx[31]]);
    let w = u32::from_le_bytes([rx[32], rx[33], rx[34], rx[35]]);
    let h = u32::from_le_bytes([rx[36], rx[37], rx[38], rx[39]]);
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut desc as *mut _);
    if va <= 0 {
        return Err("attach failed");
    }
    if w != desc.width || h != desc.height {
        let _ = mk_surface_release(handle);
        return Err("codec dim mismatch");
    }
    if (desc.width as u64) * (desc.height as u64) * 4 > desc.byte_len {
        let _ = mk_surface_release(handle);
        return Err("surface too small");
    }
    if (desc.stride as u64) < (desc.width as u64) * 4
        || (desc.stride as u64) * (desc.height as u64) > desc.byte_len
    {
        let _ = mk_surface_release(handle);
        return Err("bad surface geometry");
    }
    let px = copy_surface(va as usize, desc.stride, desc.width, desc.height);
    let _ = mk_surface_release(handle);
    Ok(Decoded { px, w: desc.width, h: desc.height })
}

fn share_input(bytes: &[u8]) -> Result<(u64, u64, *mut u8, usize), &'static str> {
    let byte_len = bytes.len() as u64;
    let padded = ((bytes.len() + 3) & !3).max(4);
    let base = mk_mmap(core::ptr::null_mut(), padded, 0x3, 0x22, -1, 0);
    if (base as isize) <= 0 {
        return Err("input mmap failed");
    }
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), base, bytes.len());
        if padded > bytes.len() {
            core::ptr::write_bytes(base.add(bytes.len()), 0, padded - bytes.len());
        }
    }
    let width = (padded / 4) as u32;
    let desc = SurfaceDescriptor {
        width,
        height: 1,
        stride: width * 4,
        format: SURFACE_FORMAT_ARGB8888,
        byte_len: padded as u64,
        base_va: base as u64,
        flags: 0,
    };
    let sid = mk_surface_register(&desc);
    if sid < 0 {
        let _ = mk_munmap(base, padded);
        return Err("input register failed");
    }
    let handle = mk_surface_share(sid as u64);
    if handle <= 0 {
        let _ = mk_munmap(base, padded);
        return Err("input share failed");
    }
    Ok((handle as u64, byte_len, base, padded))
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
