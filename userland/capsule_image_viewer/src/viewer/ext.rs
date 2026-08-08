pub fn is_codec_image(name: &[u8]) -> bool {
    let dot = match name.iter().rposition(|&b| b == b'.') {
        Some(i) if i + 1 < name.len() => i + 1,
        _ => return false,
    };
    let ext = &name[dot..];
    eq_ic(ext, b"png")
        || eq_ic(ext, b"jpg")
        || eq_ic(ext, b"jpeg")
        || eq_ic(ext, b"bmp")
        || eq_ic(ext, b"gif")
}

fn eq_ic(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_ascii_lowercase() == *y)
}
