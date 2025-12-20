use std::error::Error;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let audio_file = Assets::get("audio.mp3").ok_or("audio.mp3 not found")?;
    let cursor = std::io::Cursor::new(audio_file.data);
    let looped_decoder = rodio::Decoder::new_looped(cursor)?;
    
    sink.append(looped_decoder);
    sink.sleep_until_end(); // loops forever

    Ok(())
}