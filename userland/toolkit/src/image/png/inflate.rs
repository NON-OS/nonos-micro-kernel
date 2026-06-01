use crate::image::png::deflate::BitReader;
use crate::image::png::huffman::Huffman;
use crate::image::types::DecodeError;

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] =
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
const CL_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

fn put(out: &mut [u8], w: &mut usize, b: u8) -> Result<(), DecodeError> {
    if *w >= out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    out[*w] = b;
    *w += 1;
    Ok(())
}

fn fixed_litlen() -> Result<Huffman, DecodeError> {
    let mut l = [0u8; 288];
    for (i, e) in l.iter_mut().enumerate() {
        *e = if i < 144 {
            8
        } else if i < 256 {
            9
        } else if i < 280 {
            7
        } else {
            8
        };
    }
    Huffman::from_lengths(&l)
}

fn block(
    bits: &mut BitReader<'_>,
    ll: &Huffman,
    ds: &Huffman,
    out: &mut [u8],
    w: &mut usize,
) -> Result<(), DecodeError> {
    loop {
        let sym = ll.decode(bits)?;
        if sym < 256 {
            put(out, w, sym as u8)?;
        } else if sym == 256 {
            return Ok(());
        } else {
            let si = (sym - 257) as usize;
            if si >= 29 {
                return Err(DecodeError::Unsupported);
            }
            let len = LEN_BASE[si] as usize + bits.read_bits(LEN_EXTRA[si])? as usize;
            let dsym = ds.decode(bits)? as usize;
            if dsym >= 30 {
                return Err(DecodeError::Unsupported);
            }
            let dist = DIST_BASE[dsym] as usize + bits.read_bits(DIST_EXTRA[dsym])? as usize;
            if dist == 0 || dist > *w {
                return Err(DecodeError::Truncated);
            }
            for _ in 0..len {
                let b = out[*w - dist];
                put(out, w, b)?;
            }
        }
    }
}

fn dynamic(bits: &mut BitReader<'_>) -> Result<(Huffman, Huffman), DecodeError> {
    let hlit = bits.read_bits(5)? as usize + 257;
    let hdist = bits.read_bits(5)? as usize + 1;
    let hclen = bits.read_bits(4)? as usize + 4;
    let mut cl = [0u8; 19];
    for i in 0..hclen {
        cl[CL_ORDER[i]] = bits.read_bits(3)? as u8;
    }
    let clh = Huffman::from_lengths(&cl)?;
    let total = hlit + hdist;
    let mut lens = [0u8; 320];
    let mut i = 0usize;
    while i < total {
        let s = clh.decode(bits)?;
        match s {
            0..=15 => {
                lens[i] = s as u8;
                i += 1;
            }
            16 => {
                if i == 0 {
                    return Err(DecodeError::Unsupported);
                }
                let r = bits.read_bits(2)? as usize + 3;
                let p = lens[i - 1];
                for _ in 0..r {
                    if i >= total {
                        return Err(DecodeError::Unsupported);
                    }
                    lens[i] = p;
                    i += 1;
                }
            }
            17 => {
                let r = bits.read_bits(3)? as usize + 3;
                i += r.min(total - i);
            }
            18 => {
                let r = bits.read_bits(7)? as usize + 11;
                i += r.min(total - i);
            }
            _ => return Err(DecodeError::Unsupported),
        }
    }
    Ok((Huffman::from_lengths(&lens[..hlit])?, Huffman::from_lengths(&lens[hlit..total])?))
}

pub fn inflate_zlib(src: &[u8], out: &mut [u8]) -> Result<usize, DecodeError> {
    if src.len() < 2 {
        return Err(DecodeError::Truncated);
    }
    let mut bits = BitReader::new(&src[2..]);
    let mut w = 0usize;
    loop {
        let final_block = bits.read_bits(1)?;
        let btype = bits.read_bits(2)?;
        match btype {
            0 => {
                bits.align_byte();
                let len = bits.read_bits(16)? as usize;
                let nlen = bits.read_bits(16)?;
                if nlen != (!(len as u16)) {
                    return Err(DecodeError::BadMagic);
                }
                for _ in 0..len {
                    let b = bits.read_bits(8)? as u8;
                    put(out, &mut w, b)?;
                }
            }
            1 => {
                let ll = fixed_litlen()?;
                let ds = Huffman::from_lengths(&[5u8; 30])?;
                block(&mut bits, &ll, &ds, out, &mut w)?;
            }
            2 => {
                let (ll, ds) = dynamic(&mut bits)?;
                block(&mut bits, &ll, &ds, out, &mut w)?;
            }
            _ => return Err(DecodeError::Unsupported),
        }
        if final_block != 0 {
            return Ok(w);
        }
    }
}
