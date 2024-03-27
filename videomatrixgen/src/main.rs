use std::env::{self};

use videomatrixgen::create_json_file_from_video;

#[tokio::main]
async fn main() {
    let args = env::args();
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        println!("Usage: videomatrixgen <video_path>");
        return;
    }
    let video_path = &args[1];
    create_json_file_from_video(video_path, "output", "output.json").await;
}
