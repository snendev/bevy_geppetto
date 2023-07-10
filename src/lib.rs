use std::{
    io::{BufRead, BufReader, BufWriter, Lines},
    thread,
};

use bevy::{
    prelude::{App, DefaultPlugins, IntoSystemConfigs, PostUpdate, PreUpdate, Resource, Update},
    winit::WinitSettings,
};

// use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

pub struct Test {
    pub label: String,
    pub setup: fn(&mut App),
}

impl Test {
    pub fn new(label: String, setup: fn(&mut App)) -> Self {
        Test { label, setup }
    }

    pub fn run(&self) {
        let on_main_thread = on_main_thread();
        assert!(
            on_main_thread,
            "Integration test must be run on main thread!"
        );

        let args = Arguments::parse_args();

        println!("Running in {}-mode: {}", args.mode(), self.label,);
        let mut app = App::new();

        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc);

        if args.capture {
            let file = get_or_create_input_snapshot_file(&self.label.clone(), true);
            app.insert_resource(SnapshotWriter(BufWriter::new(file)))
                // TODO: .add_system(capture_video_snapshots)
                .add_systems(
                    PostUpdate,
                    (
                        capture_input_history_snapshot,
                        flush_file_writer
                            .before(bevy::window::close_on_esc)
                            .after(capture_input_history_snapshot),
                    ),
                );
        } else if args.replay {
            let file = get_or_create_input_snapshot_file(&self.label.clone(), false);
            app.insert_resource(SnapshotReader(BufReader::new(file).lines()))
                .add_systems(PreUpdate, replay_input_history_snapshot);
        }

        (self.setup)(&mut app);
        app.run();
    }
}
