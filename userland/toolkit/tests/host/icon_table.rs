#[path = "../../src/icons/id.rs"]
mod id;
#[path = "../../src/icons/name.rs"]
mod name;

use id::IconId;

fn main() {
    let all = IconId::ALL;
    let mut fail = 0usize;
    if all.len() != 16 {
        println!("expected 16 icons, got {}", all.len());
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
        if std::fs::metadata(&path).is_err() {
            println!("{} has no mask at {}", a.name(), path);
            fail += 1;
        }
    }
    if fail == 0 {
        println!("[ICON-TABLE] PASS");
    } else {
        println!("[ICON-TABLE] FAIL {}", fail);
        std::process::exit(1);
    }
}
