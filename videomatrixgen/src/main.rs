use std::env::{self};

use videomatrixgen::create_json_file_from_video;

#[tokio::main]
async fn main() {
    let args = env::args();
    let args: Vec<String> = args.collect();
    if args.len() < 4 {
        println!("Usage: {} <video_path> <image_path> <ouput_path>", args[0]);
        return;
    }
    let video_path = &args[1];
    let image_path = &args[2];
    let output_path = &args[3];
    create_json_file_from_video(video_path, image_path, output_path).await;
}
