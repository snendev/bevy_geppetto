use std::{
    io::{BufRead, BufReader, BufWriter, Lines},
    thread,
};

use bevy::{
    app::App,
    prelude::{IntoSystemConfig, Resource},
    winit::WinitSettings,
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod cli;
use cli::{get_or_create_snapshot_file, is_snapshot};

mod playback;
use playback::{capture_input_history_snapshot, flush_file_writer, replay_input_history_snapshot};

fn on_main_thread() -> bool {
    println!("thread name: {}", thread::current().name().unwrap());
    matches!(thread::current().name(), Some("main"))
}
pub struct Test {
    pub label: String,
    pub setup: fn(&mut App),
}

#[derive(Resource)]
pub struct SnapshotWriter(BufWriter<std::fs::File>);
#[derive(Resource)]
pub struct SnapshotReader(Lines<BufReader<std::fs::File>>);

impl Test {
    pub fn run(&self) {
        let on_main_thread = on_main_thread();
        assert!(
            on_main_thread,
            "Integration test must be run on main thread!"
        );

        let is_snapshot = is_snapshot();

        println!(
            "Running in in {}-mode: {}",
            if is_snapshot { "capture" } else { "read" },
            self.label,
        );
        let mut app = App::new();

        let file = get_or_create_snapshot_file(&self.label.clone(), is_snapshot);

        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(bevy::window::close_on_esc);

        if is_snapshot {
            app.insert_resource(SnapshotWriter(BufWriter::new(file)))
                .add_system(capture_input_history_snapshot)
                .add_system(
                    flush_file_writer
                        .before(bevy::window::close_on_esc)
                        .after(capture_input_history_snapshot),
                );
        } else {
            app.insert_resource(SnapshotReader(BufReader::new(file).lines()))
                .add_system(replay_input_history_snapshot);
        }

        (self.setup)(&mut app);
        app.run();
    }
}
