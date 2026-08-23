#[path = "../../src/icons/id.rs"]
mod id;
#[path = "../../src/icons/all.rs"]
mod all;
#[path = "../../src/icons/name.rs"]
mod name;
#[path = "../../src/icons/table.rs"]
mod table;

use id::IconId;
use table::{dim, mask};

fn main() {
    let all = IconId::ALL;
    let mut fail = 0usize;
    if all.len() != 32 {
        println!("expected 32 icons, got {}", all.len());
        fail += 1;
    }
    for (i, a) in all.iter().enumerate() {
        for b in all.iter().skip(i + 1) {
            if a.name() == b.name() {
                println!("duplicate name {}", a.name());
                fail += 1;
            }
        }
        let path = format!("../assets/icons/{}.a8", a.name());
        match std::fs::read(&path) {
            Err(e) => {
                println!("{}: {}", path, e);
                fail += 1;
            }
            Ok(bytes) => {
                if bytes != mask(*a) {
                    println!("{} is wired to the wrong mask", a.name());
                    fail += 1;
                }
                if dim(*a) * dim(*a) != bytes.len() as u32 {
                    println!("{} mask is not square", a.name());
                    fail += 1;
                }
            }
        }
    }
    if fail == 0 {
        println!("[ICON-TABLE] PASS");
    } else {
        println!("[ICON-TABLE] FAIL {}", fail);
        std::process::exit(1);
    }
}
