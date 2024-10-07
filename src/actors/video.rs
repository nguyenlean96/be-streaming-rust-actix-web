use std::process::Command;

pub fn transcode_to_adaptive_bitrate(input_path: &str, output_path: &str) {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-vf")
        .arg("scale=w=1280:h=720:force_original_aspect_ratio=decrease")
        .arg("-b:v")
        .arg("1M")
        .arg(output_path)
        .output()
        .expect("Failed to transcode video");
}