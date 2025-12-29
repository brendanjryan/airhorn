use anyhow::{Context, Result};
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::io::Cursor;

/// Play the airhorn sound using the default audio output device.
pub fn play_airhorn() -> Result<()> {
    let _stream_handle = OutputStreamBuilder::open_default_stream()
        .context("Failed to open audio output stream. Please check your audio device settings.")?;

    let cursor = Cursor::new(crate::audio::AIRHORN_DATA);

    let source = Decoder::new(cursor)
        .context("Failed to decode audio data. The embedded audio may be corrupted.")?;

    let sink = Sink::connect_new(_stream_handle.mixer());

    sink.append(source);

    sink.sleep_until_end();

    // Explicitly drop to trigger any cleanup, but stderr is suppressed by main
    drop(_stream_handle);

    Ok(())
}
