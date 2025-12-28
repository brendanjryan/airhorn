use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

/// Play the airhorn sound using the default audio output device.
pub fn play_airhorn() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .context("Failed to open audio output stream. Please check your audio device settings.")?;

    let cursor = Cursor::new(crate::audio::AIRHORN_DATA);

    let source = Decoder::new(cursor)
        .context("Failed to decode audio data. The embedded audio may be corrupted.")?;

    let sink = Sink::try_new(&stream_handle)
        .context("Failed to create audio sink. Your audio device may be busy.")?;

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
