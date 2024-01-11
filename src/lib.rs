use std::io::{BufRead, BufReader, BufWriter, Lines, Write};

use bevy::prelude::{
    resource_exists, App, Commands, Event, EventReader, IntoSystemConfigs, Last, Plugin,
    PostUpdate, PreUpdate, Res, ResMut, Resource,
};

mod systems;
use systems::{capture_input_history_snapshot, replay_input_history_snapshot};

impl SnapshotEvent {
    pub fn record(label: String) -> Self {
        Self::Record { label }
    }

    pub fn playback(label: String) -> Self {
        Self::Playback { label }
    }

    pub fn stop() -> Self {
        Self::Stop
    }
}

#[derive(Resource)]
pub struct GeppettoConfig {
    pub snapshots_dir: String,
}

impl Default for GeppettoConfig {
    fn default() -> Self {
        let snapshots_dir = std::env::var("SNAPSHOTS_DIR").unwrap_or(".snapshots".to_string());
        if !std::path::Path::new(&snapshots_dir).exists() {
            std::fs::create_dir(&snapshots_dir).unwrap();
        }
        Self { snapshots_dir }
    }
}

impl GeppettoConfig {
    pub fn directory(&self) -> &str {
        self.snapshots_dir.as_str()
    }

    pub fn make_path(&self, label: &str) -> std::path::PathBuf {
        fn sanitize_label(label: &str) -> String {
            label.replace(' ', "-").to_ascii_lowercase()
        }

        std::path::Path::new(&self.snapshots_dir)
            .join(sanitize_label(label))
            .with_extension("snapshot")
    }
}

pub struct GeppettoPlugin;

impl Plugin for GeppettoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GeppettoConfig>()
            .add_event::<SnapshotEvent>()
            .add_systems(
                PreUpdate,
                replay_input_history_snapshot.run_if(resource_exists::<SnapshotReader>()),
            )
            .add_systems(
                PostUpdate,
                capture_input_history_snapshot.run_if(resource_exists::<SnapshotWriter>()),
            )
            .add_systems(Last, handle_events);
    }
}

#[derive(Clone, Debug, Event)]
pub enum SnapshotEvent {
    Record { label: String },
    Playback { label: String },
    Stop,
}

#[derive(Resource)]
pub struct SnapshotWriter {
    pub writer: BufWriter<std::fs::File>,
    pub label: String,
}

impl SnapshotWriter {
    pub fn create(
        path: impl AsRef<std::path::Path>,
        label: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        Ok(Self {
            writer: BufWriter::new(file),
            label,
        })
    }
}

#[derive(Resource)]
pub struct SnapshotReader {
    pub reader: Lines<BufReader<std::fs::File>>,
    pub label: String,
}

impl SnapshotReader {
    pub fn create(
        path: impl AsRef<std::path::Path>,
        label: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file).lines(),
            label,
        })
    }
}

fn handle_events(
    mut commands: Commands,
    config: Res<GeppettoConfig>,
    mut events: EventReader<SnapshotEvent>,
    mut writer: Option<ResMut<SnapshotWriter>>,
    reader: Option<Res<SnapshotReader>>,
) {
    for event in events.read() {
        if reader.is_some() {
            commands.remove_resource::<SnapshotReader>();
        }
        if let Some(writer) = writer.as_mut() {
            writer
                .writer
                .flush()
                .expect("BufWriter to flush contents successfully");
            commands.remove_resource::<SnapshotWriter>();
        }
        match event {
            SnapshotEvent::Record { label } => {
                let path = config.make_path(label.as_str());
                commands.insert_resource(
                    SnapshotWriter::create(path, label.to_string())
                        .expect("writer to be a valid file path"),
                );
            }
            SnapshotEvent::Playback { label } => {
                let path = config.make_path(label.as_str());
                commands.insert_resource(
                    SnapshotReader::create(path, label.to_string())
                        .expect("reader to be a valid file path"),
                );
            }
            SnapshotEvent::Stop => {}
        };
    }
}
