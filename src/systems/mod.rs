mod inputs;
pub(crate) use inputs::{capture_input_history_snapshot, replay_input_history_snapshot};

mod screenshots;
pub(crate) use screenshots::{
    encode_gif, read_recording_rate, receive_images, save_recording_rate, take_screenshots,
    PlaybackFrames, RecordingRate,
};
