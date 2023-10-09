use super::ffmpeg::Formats;

pub fn which(binary: &str) -> bool {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("where")
            .arg(binary)
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("which")
            .arg(binary)
            .output()
            .expect("failed to execute process")
    };
    output.status.success()
}

pub fn replace_ext(path: &str, ext: &str) -> String {
    let mut path = path.to_string();
    let mut ext = ext.to_string();
    if !ext.starts_with('.') {
        ext = ".".to_string() + &ext;
    }
    if let Some(pos) = path.rfind('.') {
        path.replace_range(pos.., &ext);
    }
    path
}

pub fn from_bin_get_ext() -> (Formats, String) {
    let bin_name = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    match bin_name.as_str() {
        case if case.contains("webp") => (Formats::Webp, "webp".to_string()),
        case if case.contains("avif") => (Formats::Avif, "avif".to_string()),
        case if case.contains("x264") => (Formats::X264, "x264.mp4".to_string()),
        case if case.contains("x265") => (Formats::X265, "x265.mp4".to_string()),
        case if case.contains("x265ll") => (Formats::X265LL, "x265ll.mp4".to_string()),
        case if case.contains("x264ll") => (Formats::X264LL, "x264ll.mp4".to_string()),
        case if case.contains("prores") => (Formats::ProRes, "mov".to_string()),
        _ => {
            println!("Unknown binary name, press any key to exit...");
            std::io::stdin().read_line(&mut String::new()).unwrap();
            panic!("Unknown binary name")
        }
    }
}

pub fn ffpb_or_ffmpeg() -> String {
    let bin_name = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if which("ffpb") && bin_name.contains("ffpb") {
        "ffpb".to_string()
    } else {
        "ffmpeg".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_ext() {
        assert_eq!(replace_ext("test.png", "webp"), "test.webp".to_string());
    }
}
