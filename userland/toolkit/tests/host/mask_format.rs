const NAMES: &[&str] = &[
    "about", "audio_player", "browser", "calc", "clock", "editor", "files",
    "fs_file", "fs_folder", "image_viewer", "processes", "settings", "snake",
    "terminal", "video_player", "wallet",
];

fn main() {
    let mut fail = 0usize;
    for name in NAMES {
        let path = format!("../assets/icons/{}.a8", name);
        let bytes = match std::fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                println!("{}: {}", path, e);
                fail += 1;
                continue;
            }
        };
        let dim = (bytes.len() as f64).sqrt() as usize;
        if dim * dim != bytes.len() {
            println!("{}: len {} is not square", name, bytes.len());
            fail += 1;
        }
        if dim != 192 {
            println!("{}: dim {} expected 192", name, dim);
            fail += 1;
        }
        let ink = bytes.iter().filter(|&&a| a > 8).count();
        if ink == 0 {
            println!("{}: mask is empty", name);
            fail += 1;
        }
        if ink * 100 / bytes.len() > 60 {
            println!("{}: mask is {}% ink, art is probably filled", name, ink * 100 / bytes.len());
            fail += 1;
        }
    }
    if fail == 0 {
        println!("[MASK-FORMAT] PASS");
    } else {
        println!("[MASK-FORMAT] FAIL {}", fail);
        std::process::exit(1);
    }
}
