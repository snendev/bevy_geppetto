use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    reflect::Enum,
};
use bevy_editor_pls::{
    editor::EditorInternalState, egui_dock::NodeIndex, AddEditorWindow, EditorPlugin,
};
use bevy_geppetto::GeppettoPlugin;

use bevy_geppetto_editor_window::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        EditorPlugin::default().in_new_window(Window::default()),
        FrameTimeDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        GeppettoPlugin,
    ));

    // add the window
    app.add_editor_window::<GeppettoWindow>();
    let mut internal_state = app.world.resource_mut::<EditorInternalState>();
    internal_state.split_below::<GeppettoWindow>(NodeIndex::root().left().left(), 0.6);

    app.add_systems(Startup, ui)
        .add_systems(Update, track_keypress);

    app.run();
}

fn ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Name::new("Text"),
                TextBundle {
                    ..Default::default()
                },
            ));
        });
}

fn track_keypress(mut query: Query<&mut Text>, inputs: Res<Input<KeyCode>>) {
    let key_text = inputs
        .get_pressed()
        .next()
        .map_or("", |keycode| keycode.variant_name());

    for mut text in query.iter_mut() {
        let style = TextStyle {
            font_size: 128.,
            color: Color::rgb(0.3, 0.3, 0.3),
            ..Default::default()
        };
        *text = Text::from_section(key_text, style);
    }
}
