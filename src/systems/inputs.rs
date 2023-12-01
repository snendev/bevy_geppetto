use std::io::Write;

use bevy::{
    input::{
        gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent},
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::{
        Commands, Entity, EventReader, EventWriter, Local, Query, ResMut,
        TouchInput, With,
    },
    window::{CursorMoved, ReceivedCharacter, Window},
};

use serde::{Deserialize, Serialize};

use crate::{SnapshotReader, SnapshotWriter};

#[derive(Deserialize, Serialize)]
struct InputEventsRecord {
    pub tick: u16,
    pub characters: Vec<ReceivedCharacter>,
    pub keys: Vec<KeyboardInput>,
    pub mouse_buttons: Vec<MouseButtonInput>,
    pub mouse_wheel: Vec<MouseWheel>,
    pub mouse_motion: Vec<MouseMotion>,
    pub cursor_motion: Vec<CursorMoved>,
    pub gamepad_connection: Vec<GamepadConnectionEvent>,
    pub gamepad_axis: Vec<GamepadAxisChangedEvent>,
    pub gamepad_button: Vec<GamepadButtonChangedEvent>,
    pub touch: Vec<TouchInput>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn capture_input_history_snapshot(
    mut snapshot_file: ResMut<SnapshotWriter>,
    mut tick_count: Local<u16>,
    // keyboard
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    // mouse
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    // gamepad
    mut gamepad_connection_events: EventReader<GamepadConnectionEvent>,
    mut gamepad_axis_events: EventReader<GamepadAxisChangedEvent>,
    mut gamepad_button_events: EventReader<GamepadButtonChangedEvent>,
    // touch
    mut touch_events: EventReader<TouchInput>,
    // mut touchpad_magnify_events: EventReader<TouchpadMagnify>, // bevy 0.11
    // mut touchpad_rotate_events: EventReader<TouchpadRotate>, // bevy 0.11
) {
    let record = InputEventsRecord {
        tick: *tick_count,
        characters: char_input_events.read().cloned().collect(),
        keys: keyboard_input_events.read().cloned().collect(),
        mouse_buttons: mouse_button_input_events.read().cloned().collect(),
        mouse_wheel: mouse_wheel_events.read().cloned().collect(),
        mouse_motion: mouse_motion_events.read().cloned().collect(),
        cursor_motion: cursor_moved_events.read().cloned().collect(),
        gamepad_connection: gamepad_connection_events.read().cloned().collect(),
        gamepad_axis: gamepad_axis_events.read().cloned().collect(),
        gamepad_button: gamepad_button_events.read().cloned().collect(),
        touch: touch_events.read().cloned().collect(),
    };
    *tick_count += 1;

    let text = ron::ser::to_string(&record).unwrap();
    snapshot_file.0.write_all(text.as_bytes()).unwrap();
    snapshot_file.0.write_all(b"\n").unwrap();
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn replay_input_history_snapshot(
    mut commands: Commands,
    windows: Query<Entity, With<Window>>,
    mut snapshot_file: ResMut<SnapshotReader>,
    // keyboard
    mut char_input_events: EventWriter<ReceivedCharacter>,
    mut keyboard_input_events: EventWriter<KeyboardInput>,
    // mouse
    mut mouse_button_input_events: EventWriter<MouseButtonInput>,
    mut mouse_wheel_events: EventWriter<MouseWheel>,
    mut mouse_motion_events: EventWriter<MouseMotion>,
    mut cursor_moved_events: EventWriter<CursorMoved>,
    // gamepad
    mut gamepad_connection_events: EventWriter<GamepadConnectionEvent>,
    mut gamepad_axis_events: EventWriter<GamepadAxisChangedEvent>,
    mut gamepad_button_events: EventWriter<GamepadButtonChangedEvent>,
    // touch
    mut touch_events: EventWriter<TouchInput>,
    // mut touchpad_magnify_events: EventReader<TouchpadMagnify>, // bevy 0.11
    // mut touchpad_rotate_events: EventReader<TouchpadRotate>, // bevy 0.11
) {
    if let Some(history) = snapshot_file.0.next() {
        let record: InputEventsRecord = ron::de::from_str(&history.unwrap()).unwrap();
        char_input_events.send_batch(record.characters);
        keyboard_input_events.send_batch(record.keys);
        mouse_button_input_events.send_batch(record.mouse_buttons);
        mouse_wheel_events.send_batch(record.mouse_wheel);
        mouse_motion_events.send_batch(record.mouse_motion);
        cursor_moved_events.send_batch(record.cursor_motion);
        gamepad_connection_events.send_batch(record.gamepad_connection);
        gamepad_axis_events.send_batch(record.gamepad_axis);
        gamepad_button_events.send_batch(record.gamepad_button);
        touch_events.send_batch(record.touch);
    } else {
        for window in windows.iter() {
            commands.entity(window).despawn();
        }
    }
}
