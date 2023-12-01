use std::{fs::File, io::Write, time::Duration};

use serde::{Deserialize, Serialize};

use bevy::{
    prelude::{Commands, Entity, Local, NextState, Query, Res, ResMut, Resource, Time, With},
    render::{texture::Image, view::screenshot::ScreenshotManager},
    window::PrimaryWindow,
};

use crate::{
    directory::{get_gifs_dir, get_screenshots_dir},
    SnapshotReader, SnapshotWriter, TestConfiguration, TestState,
};

#[derive(Resource, Serialize, Deserialize)]
pub(crate) struct RecordingRate(pub(crate) Duration);

impl RecordingRate {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
}

impl Default for RecordingRate {
    fn default() -> Self {
        Self::new(Duration::from_millis(500))
    }
}

pub enum PlaybackFrame {
    Pending(crossbeam_channel::Receiver<Image>),
    Resolved(Image),
}

impl PlaybackFrame {
    fn poll(&self) -> Option<Image> {
        if let Self::Pending(rx) = self {
            if let Ok(image) = rx.try_recv() {
                return Some(image);
            }
        }
        None
    }

    fn get(&self) -> Option<&Image> {
        match self {
            Self::Resolved(image) => Some(image),
            _ => None,
        }
    }
}

#[derive(Default, Resource)]
pub struct PlaybackFrames {
    frames: Vec<PlaybackFrame>,
    size: (u16, u16),
}

// there is no way that this doesn't have weird async bugs.
pub(crate) fn take_screenshots(
    primary_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut gif_frames: ResMut<PlaybackFrames>,
    mut captured_frame: Local<u32>,
    time: Res<Time>,
    recording_rate: Res<RecordingRate>,
    mut elapsed: Local<Duration>,
) {
    *elapsed += time.delta();

    if *elapsed > recording_rate.0 {
        *elapsed -= recording_rate.0;
    } else {
        return;
    }

    // clone the Arcs so that the thread can decrement the pending screenshot count
    // and also access the gif encoder
    let (tx, rx) = crossbeam_channel::bounded(1000);
    gif_frames.frames.push(PlaybackFrame::Pending(rx));

    // take the screenshot
    screenshot_manager
        .take_screenshot(primary_window.single(), move |image| {
            // send the image back to the ECS
            tx.send(image).unwrap();
        })
        .unwrap();

    *captured_frame += 1;
}

pub(crate) fn receive_images(
    mut gif_frames: ResMut<PlaybackFrames>,
    config: Res<TestConfiguration>,
) {
    let current_size = gif_frames.size;
    let mut updated_size = None;

    for (index, frame) in gif_frames.frames.iter_mut().enumerate() {
        let path = get_screenshots_dir().join(format!("{}-{}.png", config.sanitized_label, index));
        if let Some(image) = frame.poll() {
            let width: u16 = image.width().try_into().unwrap();
            let height: u16 = image.height().try_into().unwrap();
            let x = if width > current_size.0 {
                Some(width)
            } else {
                None
            };
            let y = if height > current_size.1 {
                Some(height)
            } else {
                None
            };
            updated_size = x.zip(y).or(updated_size);
            let image_clone = image.clone();
            std::thread::spawn(move || {
                image_clone.try_into_dynamic().unwrap().save(path).unwrap();
            });
            *frame = PlaybackFrame::Resolved(image);
        }
    }
    if let Some(size) = updated_size {
        gif_frames.size = size;
    }
}

pub(crate) fn save_recording_rate(
    recording_rate: Res<RecordingRate>,
    mut snapshot_file: ResMut<SnapshotWriter>,
) {
    let text = ron::ser::to_string(&*recording_rate).unwrap();
    snapshot_file.0.write_all(text.as_bytes()).unwrap();
    snapshot_file.0.write_all(b"\n").unwrap();
}

pub(crate) fn encode_gif(
    mut frames: ResMut<PlaybackFrames>,
    config: Res<TestConfiguration>,
    mut state: ResMut<NextState<TestState>>,
) {
    if !frames.frames.iter().all(|frames| frames.get().is_some()) {
        return;
    }
    let gif_file =
        File::create(get_gifs_dir().join(format!("{}.gif", config.sanitized_label))).unwrap();

    let size = frames.size;

    let mut encoder = gif::Encoder::new(gif_file, size.0, size.1, &[]).unwrap();
    for frame in frames.frames.drain(..) {
        let PlaybackFrame::Resolved(mut image) = frame else {
            continue;
        };
        encoder
            .write_frame(&gif::Frame::from_rgba(
                image.width().try_into().unwrap(),
                image.height().try_into().unwrap(),
                &mut image.data,
            ))
            .unwrap();
    }

    state.set(TestState::Shutdown);
}

pub(crate) fn read_recording_rate(
    mut commands: Commands,
    mut snapshot_file: ResMut<SnapshotReader>,
) {
    let line = snapshot_file.0.next().unwrap().unwrap();
    let recording_rate: RecordingRate = ron::de::from_str(&line).unwrap();
    commands.insert_resource(recording_rate);
}
