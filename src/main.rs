mod ffmpeg;
mod helper;
mod notification;
mod path;
use std::thread::spawn;

fn main() {
    if !ffmpeg::check_ffmpeg().unwrap() {
        helper::func::input(helper::msg::ffmpeg_not_found());
        return;
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        helper::func::input(helper::msg::no_file_found());
        return;
    }

    let mut threads = Vec::new();
    for i in args.iter().skip(1) {
        let arg = i.clone();
        threads.push(spawn(move || {
            ffmpeg::convert(&arg).unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    match notification::notify("Done!") {
        Ok(_) => {}
        Err(e) => {
            helper::func::input(e.to_string().as_str());
        }
    };
}
