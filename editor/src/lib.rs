use bevy::prelude::*;
use bevy_editor_pls::{
    editor_window::{EditorWindow, EditorWindowContext},
    egui_dock::egui,
};
use bevy_geppetto::*;

pub enum EditorOpenSetting {
    Windowed,
    FullScreen,
}

const DEFAULT_FILENAME: &str = "snapshot";

#[derive(Default)]
pub struct GeppettoWindowState {
    filename: String,
}

pub struct GeppettoWindow;

impl EditorWindow for GeppettoWindow {
    type State = GeppettoWindowState;
    const NAME: &'static str = "Geppetto";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<GeppettoWindow>().unwrap();

        ui.horizontal(|ui| {
            egui::TextEdit::singleline(&mut state.filename)
                .hint_text(DEFAULT_FILENAME)
                .desired_width(120.0)
                .show(ui);

            let enter_pressed: bool = ui.input(|input| input.key_pressed(egui::Key::Enter));

            if ui.button("Record").clicked() || enter_pressed {
                let filename = if state.filename.is_empty() {
                    DEFAULT_FILENAME
                } else {
                    &state.filename
                };
                world.send_event(SnapshotEvent::record(filename.to_string()));
            }
        });

        let active_snapshot_label = world
            .get_resource::<SnapshotWriter>()
            .map(|SnapshotWriter { label, .. }| format!("Recording: {}", label))
            .or(world
                .get_resource::<SnapshotReader>()
                .map(|SnapshotReader { label, .. }| format!("Replaying: {}", label)));

        if let Some(label) = active_snapshot_label {
            ui.label(egui::RichText::new(label).color(egui::Color32::GREEN));
            if ui.button("Stop").clicked() {
                world.send_event(SnapshotEvent::Stop);
            }
        }

        let config = world
            .get_resource::<GeppettoConfig>()
            .expect("GeppettoPlugin to have attached a GeppettoConfig");

        let full_path = std::path::Path::new(config.directory());
        let directory = std::fs::read_dir(full_path).unwrap_or_else(|_| {
            std::fs::create_dir(full_path).unwrap();
            std::fs::read_dir(full_path).unwrap()
        });

        for entry in directory {
            let entry = entry.unwrap();
            let path = entry.path();
            let bare_path = path.with_extension("");
            let filename = bare_path
                .file_name()
                .expect("path to have a filename")
                .to_str()
                .expect("path to be a valid filename");

            ui.horizontal(|ui| {
                ui.label(filename);
                if ui.button("Play").clicked() {
                    world.send_event(SnapshotEvent::playback(filename.to_string()));
                }
            });
        }
    }
}
