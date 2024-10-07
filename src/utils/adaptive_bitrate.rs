use std::process::{Command, Stdio};
use std::path::Path;

// Function to generate multiple bitrate streams using FFmpeg
pub fn generate_adaptive_bitrate(input_path: &str, output_dir: &str) -> Result<(), String> {
    let output_dir_path = Path::new(output_dir);
    if !output_dir_path.exists() {
        return Err(format!("Output directory does not exist: {}", output_dir));
    }

    // Create different resolution streams
    let resolutions = vec!["480p", "720p", "1080p"];
    let bitrates = vec!["500k", "1500k", "3000k"];

    for (res, bitrate) in resolutions.iter().zip(bitrates.iter()) {
        let output_file = format!("{}/stream_{}.m3u8", output_dir, res);
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-vf")
            .arg(format!("scale=-2:{}", res_to_height(res)))
            .arg("-b:v")
            .arg(bitrate)
            .arg("-hls_time")
            .arg("4")
            .arg("-hls_playlist_type")
            .arg("vod")
            .arg("-f")
            .arg("hls")
            .arg(&output_file)
            .stdout(Stdio::null())
            .status();

        match status {
            Ok(status) if status.success() => {
                println!("Successfully created {} stream at {} bitrate", res, bitrate);
            }
            Ok(_) | Err(_) => {
                return Err(format!("Failed to create {} stream at {} bitrate", res, bitrate));
            }
        }
    }

    Ok(())
}

// Helper function to map resolution to height for scaling
fn res_to_height(resolution: &str) -> &str {
    match resolution {
        "480p" => "480",
        "720p" => "720",
        "1080p" => "1080",
        _ => "720", // Default to 720p if unknown
    }
}