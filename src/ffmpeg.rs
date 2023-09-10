use super::path::{ffpb_or_ffmpeg, from_bin_get_ext, replace_ext};
use std::process::Command;

pub enum Formats {
    Webp,
    Avif,
    X264,
    X265,
    X265LL,
    ProRes,
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
    pub fn x265_ll<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-c:v", "libx265", "-vf", "scale=ceil(iw/2)*2:ceil(ih/2)*2", "-pix_fmt", "yuv420p", "-tag:v", "hvc1", "-crf", "0", "-c:a", "copy", out_path]
    }
    pub fn prores<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
        vec!["-i", in_path, "-c:v", "prores_ks", "-profile:v", "5", "-pix_fmt", "yuva444p10le", "-c:a", "pcm_s24le", out_path]
    }
}

pub fn check_ffmpeg() -> Result<bool, std::io::Error> {
    let output = Command::new("ffmpeg").arg("-version").output()?;
    Ok(output.status.success())
}

fn get_args(in_path: &str) -> Vec<String> {
    let (format, ext) = from_bin_get_ext();
    let out_path = replace_ext(in_path, &ext);
    match format {
        Formats::Webp => params::webp(in_path, &out_path),
        Formats::Avif => params::avif(in_path, &out_path),
        Formats::X264 => params::x264(in_path, &out_path),
        Formats::X265 => params::x265(in_path, &out_path),
        Formats::X265LL => params::x265_ll(in_path, &out_path),
        Formats::ProRes => params::prores(in_path, &out_path),
    }
    .iter()
    .map(|&x| x.to_string())
    .collect()
}

pub fn convert(in_path: &str) -> Result<bool, std::io::Error> {
    let output = Command::new(ffpb_or_ffmpeg())
        .args(get_args(in_path))
        .spawn()?
        .wait()?;
    Ok(output.success())
}
