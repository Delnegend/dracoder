use super::path::{from_bin_get_ext, replace_ext};
use std::process::Command;

pub enum Formats {
    Webp,
    Avif,
    X264,
    X265,
}

#[rustfmt::skip]
mod params {
    pub fn webp<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-vcodec", "libwebp", "-compression_level", "6", "-q:v", "80", out_path]
    }
    pub fn avif<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-c:v", "libsvtav1", "-crf", "22", "-preset", "6", "-vf", "scale=ceil(iw/2)*2:ceil(ih/2)*2", out_path]
    }
    pub fn x264<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-c:v", "libx264", "-vf", "scale=ceil(iw/2)*2:ceil(ih/2)*2", "-pix_fmt", "yuv420p", "-crf", "18", "-c:a", "copy", out_path]
    }
    pub fn x265<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-c:v", "libx265", "-vf", "scale=ceil(iw/2)*2:ceil(ih/2)*2", "-pix_fmt", "yuv420p", "-tag:v", "hvc1", "-crf", "18", "-preset", "slow", "-c:a", "copy", out_path]
    }
}

pub fn check_ffmpeg() -> Result<bool, std::io::Error> {
    let output = Command::new("ffmpeg").arg("-version").output()?;
    Ok(output.status.success())
}

fn get_args(in_path: &str) -> Vec<String> {
    let format = from_bin_get_ext();
    let out_path = match format {
        Formats::Webp => replace_ext(in_path, "webp"),
        Formats::Avif => replace_ext(in_path, "avif"),
        Formats::X264 => replace_ext(in_path, ".x264.mp4"),
        Formats::X265 => replace_ext(in_path, ".x265.mp4"),
    };
    match format {
        Formats::Webp => params::webp(in_path, &out_path),
        Formats::Avif => params::avif(in_path, &out_path),
        Formats::X264 => params::x264(in_path, &out_path),
        Formats::X265 => params::x265(in_path, &out_path),
    }
    .iter()
    .map(|&x| x.to_string())
    .collect()
}

pub fn convert(in_path: &str) -> Result<bool, std::io::Error> {
    let output = Command::new("ffmpeg")
        .args(get_args(in_path))
        .spawn()?
        .wait()?;
    Ok(output.success())
}
