use std::{
    io::{BufRead, BufReader, BufWriter, Lines, Write},
    thread,
    time::Duration,
};

use bevy::{
    ecs::schedule::OnEnter,
    log::{info, Level, LogPlugin},
    prelude::{
        in_state, App, Commands, DefaultPlugins, Entity, Input, IntoSystemConfigs, KeyCode,
        NextState, PluginGroup, PostUpdate, PreUpdate, Query, Res, ResMut, Resource, Startup,
        States, Update, Window, WindowPlugin, With,
    },
    winit::WinitSettings,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) mod directory;
use crate::{directory::get_screenshots_dir, systems::encode_gif};

mod cli;
use cli::Arguments;

mod files;
use files::{get_or_create_input_snapshot_file, sanitize_label};

mod systems;
use systems::{
    capture_input_history_snapshot, read_recording_rate, receive_images,
    replay_input_history_snapshot, save_recording_rate, take_screenshots, PlaybackFrames,
    RecordingRate,
};

fn on_main_thread() -> bool {
    println!("thread name: {}", thread::current().name().unwrap());
    matches!(thread::current().name(), Some("main"))
}

#[derive(Resource)]
pub(crate) struct SnapshotWriter(BufWriter<std::fs::File>);
#[derive(Resource)]
pub(crate) struct SnapshotReader(Lines<BufReader<std::fs::File>>);

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum TestState {
    #[default]
    Active,
    PendingClose,
    Shutdown,
}

// modelled after bevy::window::close_on_ecs, but sets state to PendingClose instead
pub(crate) fn begin_unwinding_on_escape(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<TestState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(TestState::PendingClose);
    }
}

// add this system before any system that closes the app
pub(crate) fn flush_file_writers(mut snapshot_file: ResMut<SnapshotWriter>) {
    snapshot_file.0.flush().unwrap();
}

// despawns all windows to close the application
pub(crate) fn close_windows(mut commands: Commands, windows: Query<Entity, With<Window>>) {
    for window in windows.iter() {
        commands.entity(window).despawn();
    }
}

#[derive(Resource)]
pub struct TestConfiguration {
    pub label: String,
    pub sanitized_label: String,
}

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
        let screenshots_dir = get_screenshots_dir();

        println!("Running in {}-mode: {}", args.mode(), self.label);
        let mut app = App::new();

        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..Default::default()
        })
        .insert_resource(TestConfiguration {
            label: self.label.clone(),
            sanitized_label: sanitize_label(self.label.as_str()),
        })
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..Default::default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Test - {}", self.label),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::new());

        if let Some(recording_rate) = args.capture {
            // first delete any existing screenshots
            std::fs::remove_dir_all(&screenshots_dir).unwrap();

            let file = get_or_create_input_snapshot_file(&self.label.clone(), true);
            app.insert_resource(SnapshotWriter(BufWriter::new(file)))
                .init_resource::<PlaybackFrames>()
                .insert_resource(RecordingRate::new(Duration::from_millis(recording_rate)))
                .add_state::<TestState>()
                .add_systems(Startup, save_recording_rate)
                .add_systems(
                    PostUpdate,
                    (
                        begin_unwinding_on_escape,
                        capture_input_history_snapshot,
                        take_screenshots,
                    )
                        .chain()
                        .run_if(in_state(TestState::Active)),
                )
                .add_systems(OnEnter(TestState::PendingClose), flush_file_writers)
                .add_systems(PostUpdate, receive_images)
                .add_systems(
                    PostUpdate,
                    encode_gif.run_if(in_state(TestState::PendingClose)),
                )
                .add_systems(OnEnter(TestState::Shutdown), close_windows);
        } else if args.replay {
            let file = get_or_create_input_snapshot_file(&self.label.clone(), false);
            app.insert_resource(SnapshotReader(BufReader::new(file).lines()))
                .add_systems(Startup, read_recording_rate)
                .add_systems(PreUpdate, replay_input_history_snapshot);
        } else {
            app.add_systems(Update, bevy::window::close_on_esc);
        }

        (self.setup)(&mut app);
        app.run();

        // once app is cleaned up, check the screenshots!

        info!(
            "Screenshots taken: {}, view gif",
            std::fs::read_dir(&screenshots_dir)
                .map(|iter| iter.count())
                .unwrap_or(0),
        );
    }
}
