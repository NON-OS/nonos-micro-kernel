// GIF interlace stores rows in four passes; map a pass-order row index to its
// real y within a frame of `height` rows.
pub(super) fn deinterlaced_row(pass_row: usize, height: usize) -> Option<usize> {
    let mut seen = 0usize;
    for (start, step) in [(0usize, 8usize), (4, 8), (2, 4), (1, 2)] {
        let mut y = start;
        while y < height {
            if seen == pass_row {
                return Some(y);
            }
            seen += 1;
            y += step;
        }
    }
    None
}
