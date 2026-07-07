use std::path::Path;

pub fn root_dirs() -> &'static [&'static str] {
    &["src", "userland"]
}

pub fn skip(path: &Path) -> bool {
    let p = path.to_string_lossy();
    p.contains("/target/")
        || p.contains("/fs_proofs/")
        || p.contains("/crypto_proofs/")
        || p.ends_with("/build.rs")
}

pub fn is_rust(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
}
