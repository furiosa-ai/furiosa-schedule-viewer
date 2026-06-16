use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dist = manifest_dir.join("dist");

    let index = dist.join("index.html");
    if !index.is_file() {
        panic!(
            "\n\
             Could not find pre-built web assets.\n\
             Looked for: {}\n\
             \n\
             The `dist/` directory must be present at the crate root.\n\
             Obtain it from the furiosa-schedule-viewer project (run `make sync`\n\
             there) and commit it alongside this crate.\n",
            index.display(),
        );
    }

    walk_rerun(&dist);
}

fn walk_rerun(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rerun(&path);
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}