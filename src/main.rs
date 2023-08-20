mod ffmpeg;
mod helper;
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
    for i in 1..args.len() {
        let in_path = args[i].clone();
        let out_path = path::replace_ext(&in_path, path::from_bin_get_ext());
        threads.push(spawn(move || {
            ffmpeg::convert(&in_path, &out_path).unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
