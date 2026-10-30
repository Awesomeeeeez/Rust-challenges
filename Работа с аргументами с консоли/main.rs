#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_definition: bool,
}

use std::env;

fn collect_settings() -> Settings {
    let mut args = env::args().skip(1).take(3);

    let video_file = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified");
        std::process::exit(1);
    });

    let subtitles = try_get_bool(&mut args);
    let high_definition = try_get_bool(&mut args);

    Settings {
        video_file,
        subtitles,
        high_definition,
    }
}

fn try_get_bool(args: &mut impl Iterator<Item = String>) -> bool {
    args.next()
        .map(|s| s.parse::<bool>().unwrap_or(false))
        .unwrap_or(false)
}

fn main() {
    let settings = collect_settings();
    println!("{:?}", settings);
}
