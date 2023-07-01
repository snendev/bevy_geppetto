use std::time::Duration;

use bevy::prelude::{Added, Camera, Entity, Query, Res};
use bevy_capture_media::MediaCapture;
use clap::Parser;

use crate::{
    directory::{get_input_snapshots_dir, get_video_snapshots_dir},
    TestLabel,
};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    snapshot: bool,
}

pub fn is_snapshot() -> bool {
    let Cli { snapshot } = Cli::parse();
    snapshot
}

fn sanitize_label(label: &str) -> String {
    label.replace(" ", "-").to_ascii_lowercase()
}

pub fn get_or_create_input_snapshot_file(label: &str, snapshot: bool) -> std::fs::File {
    let snapshots_dir = get_input_snapshots_dir();
    let snapshot_filename = format!("{}.ron", sanitize_label(label));
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

const TRACKING_ID: usize = 1000;

// system that captures video snapshots
pub fn capture_video_snapshots(
    mut capture: MediaCapture,
    test_label: Res<TestLabel>,
    cameras_query: Query<Entity, Added<Camera>>,
) {
    let entity = cameras_query.get_single().unwrap();
    // TODO: maybe pass a timer instead...
    capture.start_tracking_camera(TRACKING_ID, entity, Duration::from_secs(30));

    let snapshots_dir = get_video_snapshots_dir();
    let snapshot_filename = test_label.0.clone().replace(" ", "-").to_ascii_lowercase();
    let path = std::path::Path::new(&snapshots_dir).join(&snapshot_filename);
    capture.capture_gif_with_path(TRACKING_ID, path)
}
