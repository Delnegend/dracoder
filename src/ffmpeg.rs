use super::path::from_bin_get_ext;
use std::process::Command;

pub fn check_ffmpeg() -> Result<bool, std::io::Error> {
    let output = Command::new("ffmpeg").arg("-version").output()?;
    Ok(output.status.success())
}

fn get_args<'a>(in_path: &'a str, out_path: &'a str) -> Vec<&'a str> {
    match from_bin_get_ext() {
        "webp" => {
            vec![
                "-i",
                in_path,
                "-vcodec",
                "libwebp",
                "-compression_level",
                "6",
                "-q:v",
                "80",
                out_path,
            ]
        }
        "avif" => {
            vec![
                "-i",
                in_path,
                "-c:v",
                "libsvtav1",
                "-crf",
                "22",
                "-preset",
                "6",
                "-vf",
                "scale=ceil(iw/2)*2:ceil(ih/2)*2",
                out_path,
            ]
        }
        _ => {
            println!("Unknown binary name, press any key to exit...");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            panic!("Unknown binary name")
        }
    }
}

pub fn convert(in_path: &str, out_path: &str) -> Result<bool, std::io::Error> {
    let output = Command::new("ffmpeg")
        .args(get_args(in_path, out_path))
        .output()?;

    Ok(output.status.success())
}
