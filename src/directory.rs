use std::path::Path;

pub(crate) fn get_snapshots_dir() -> String {
    let snapshots_dir = std::env::var("SNAPSHOTS_DIR").unwrap_or("snapshots".to_string());
    if !Path::new(&snapshots_dir).exists() {
        std::fs::create_dir(&snapshots_dir).unwrap();
    }
    snapshots_dir
}

pub(crate) fn get_input_snapshots_dir() -> std::path::PathBuf {
    let snapshots_dir = get_snapshots_dir();
    let inputs_dir = Path::new(&snapshots_dir).join("inputs");
    if !inputs_dir.exists() {
        std::fs::create_dir(&inputs_dir).unwrap();
    }
    inputs_dir
}

pub(crate) fn _get_screenshots_dir() -> std::path::PathBuf {
    let snapshots_dir = get_snapshots_dir();
    let videos_dir = Path::new(&snapshots_dir).join("screenshots");
    if !videos_dir.exists() {
        std::fs::create_dir(&videos_dir).unwrap();
    }
    videos_dir
}
