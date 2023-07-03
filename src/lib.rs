use std::{
    io::{BufRead, BufReader, BufWriter, Lines},
    thread,
};

use bevy::{
    app::App,
    prelude::{Component, IntoSystemConfig, Resource},
    winit::WinitSettings,
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

// use bevy_capture_media::BevyCapturePlugin;

pub(crate) mod directory;

mod cli;
use cli::Arguments;

mod playback;
use playback::{capture_input_history_snapshot, flush_file_writer, replay_input_history_snapshot};

mod snapshots;
use snapshots::get_or_create_input_snapshot_file;

fn on_main_thread() -> bool {
    println!("thread name: {}", thread::current().name().unwrap());
    matches!(thread::current().name(), Some("main"))
}

#[derive(Resource)]
pub(crate) struct SnapshotWriter(BufWriter<std::fs::File>);
#[derive(Resource)]
pub(crate) struct SnapshotReader(Lines<BufReader<std::fs::File>>);
#[derive(Resource)]
pub(crate) struct TestMetadata {
    label: String,
    // duration: u64,
}

#[derive(Component)]
pub struct CameraTracker;

pub struct Test {
    pub label: String,
    pub setup: fn(&mut App),
    // pub capture_duration: u64,
}

impl Test {
    pub fn run(&self) {
        let on_main_thread = on_main_thread();
        assert!(
            on_main_thread,
            "Integration test must be run on main thread!"
        );

        let Arguments { capture, replay } = Arguments::parse_args();

        println!(
            "Running in in {}-mode: {}",
            if capture {
                "capture"
            } else if replay {
                "replay"
            } else {
                "sandbox"
            },
            self.label,
        );
        let mut app = App::new();

        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..Default::default()
        })
        .insert_resource(TestMetadata {
            label: self.label.clone(),
            // duration: self.capture_duration,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // TODO: .add_plugin(BevyCapturePlugin)
        .add_system(bevy::window::close_on_esc);

        if capture {
            let file = get_or_create_input_snapshot_file(&self.label.clone(), true);
            app.insert_resource(SnapshotWriter(BufWriter::new(file)))
                // TODO: .add_system(capture_video_snapshots)
                .add_system(capture_input_history_snapshot)
                .add_system(
                    flush_file_writer
                        .before(bevy::window::close_on_esc)
                        .after(capture_input_history_snapshot),
                );
        } else if replay {
            let file = get_or_create_input_snapshot_file(&self.label.clone(), false);
            app.insert_resource(SnapshotReader(BufReader::new(file).lines()))
                .add_system(replay_input_history_snapshot);
        }

        (self.setup)(&mut app);
        app.run();
    }
}
