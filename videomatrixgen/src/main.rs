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
        .get_matches();

    let video_path = match_result.get_one::<String>("video_path").unwrap();
    let output = match_result.get_one::<String>("output_path").unwrap();

    create_json_file_from_video(video_path, "frames", output).await;
}
