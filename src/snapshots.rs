use std::time::Duration;

use bevy::prelude::{Added, Entity, Query, Res};
// use bevy_capture_media::MediaCapture;
use clap::Parser;

use crate::{
    directory::{get_input_snapshots_dir, get_video_snapshots_dir},
    CameraTracker, TestMetadata,
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

pub(crate) fn get_or_create_input_snapshot_file(label: &str, snapshot: bool) -> std::fs::File {
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
    .expect("Missing snapshot file. Don't forget to run in snapshot mode with the -s flag");

    file
}

const TRACKING_ID: usize = 1000;

// system that captures video snapshots
// pub(crate) fn capture_video_snapshots(
//     mut capture: MediaCapture,
//     test_metadata: Res<TestMetadata>,
//     cameras_query: Query<Entity, Added<CameraTracker>>,
// ) {
//     for entity in cameras_query.iter() {
//         // TODO: maybe pass a timer instead...
//         capture.start_tracking_camera(
//             TRACKING_ID,
//             entity,
//             Duration::from_secs(test_metadata.duration),
//         );

//         let snapshots_dir = get_video_snapshots_dir();
//         let snapshot_filename = format!("{}.gif", sanitize_label(&test_metadata.label));

//         let path = std::path::Path::new(&snapshots_dir).join(&snapshot_filename);
//         println!(
//             "Recording gif of test snapshot at {}",
//             std::env::current_dir()
//                 .unwrap()
//                 .as_path()
//                 .join(&path)
//                 .to_str()
//                 .unwrap(),
//         );

//         capture.capture_gif_with_path(TRACKING_ID, path);
//     }
// }
