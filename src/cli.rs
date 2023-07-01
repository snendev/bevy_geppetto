use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    snapshot: bool,
}

pub fn is_snapshot() -> bool {
    let Cli { snapshot } = Cli::parse();
    snapshot
}

pub fn get_or_create_snapshot_file(label: &str, snapshot: bool) -> std::fs::File {
    let snapshots_dir = std::env::var("SNAPSHOTS_DIR").unwrap_or("snapshots".to_string());
    let snapshot_filename = format!("{}.ron", label.replace(" ", "-").to_ascii_lowercase());
    if !std::path::Path::new(&snapshots_dir).exists() {
        std::fs::create_dir(&snapshots_dir).unwrap();
    }
    let path = std::path::Path::new(&snapshots_dir).join(&snapshot_filename);
    println!(
        "{} snapshot at {}",
        if snapshot { "Saving" } else { "Opening" },
        std::env::current_dir()
            .unwrap()
            .as_path()
            .join(&path)
            .to_str()
            .unwrap(),
    );
    let path = path.to_str().unwrap();
    let file = if snapshot {
        std::fs::File::create(path)
    } else {
        std::fs::File::open(path)
    }
    .unwrap();

    file
}
