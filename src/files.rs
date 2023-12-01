use crate::directory::get_input_snapshots_dir;

pub(crate) fn sanitize_label(label: &str) -> String {
    label.replace(' ', "-").to_ascii_lowercase()
}

// TODO: refactor this into a system and read sanitized_label from TestConfiguration
pub(crate) fn get_or_create_input_snapshot_file(label: &str, capture: bool) -> std::fs::File {
    let snapshots_dir = get_input_snapshots_dir();
    let snapshot_filename = format!("{}.snapshot", sanitize_label(label));
    let path = std::path::Path::new(&snapshots_dir).join(snapshot_filename);
    println!(
        "{} snapshot at {}",
        if capture { "Saving" } else { "Opening" },
        std::env::current_dir()
            .unwrap()
            .as_path()
            .join(&path)
            .to_str()
            .unwrap(),
    );
    let path = path.to_str().unwrap();

    if capture {
        std::fs::File::create(path)
    } else {
        std::fs::File::open(path)
    }
    .expect("Missing snapshot file. Don't forget to run in capture mode with the -c flag")
}
