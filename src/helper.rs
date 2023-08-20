pub mod msg {
    pub fn ffmpeg_not_found() -> &'static str {
        "ffmpeg not found in PATH, press any key to exit..."
    }
    pub fn no_file_found() -> &'static str {
        "No file found, press any key to exit..."
    }
}

pub mod func {
    pub fn input(msg: &str) -> String {
        println!("{}", msg);
        std::io::stdin()
            .read_line(&mut String::new())
            .unwrap()
            .to_string()
    }
}
