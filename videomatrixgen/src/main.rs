use clap::{command, Arg};
use videomatrixgen::create_json_file_from_video;

#[tokio::main]
async fn main() {
    let match_result = command!()
        .about("Creates a matrix distance file from a video")
        .arg(
            Arg::new("video_path")
                .help("Path to the video file")
                .required(true),
        )
        .arg(
            Arg::new("output_path")
                .short('o')
                .long("output")
                .help("Path to the output file")
                .default_value("output.json"),
        )
        .arg(
            Arg::new("fps")
                .long("fps")
                .help("Frames per second")
                .default_value("1"),
        )
        .get_matches();
    // Get the video path
    let video_path = match_result.get_one::<String>("video_path").unwrap();
    // Get the output path
    let output = match_result.get_one::<String>("output_path").unwrap();
    // Get the frame per second
    let frame_nb = match_result
        .get_one::<String>("fps")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    create_json_file_from_video(video_path, "frames", output, &frame_nb).await;
}
