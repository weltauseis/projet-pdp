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
                .help("Path to the output file"),
        )
        .get_matches();

    let video_path = match_result.get_one::<String>("video_path").unwrap();
    let default_output = &"output.json".to_string();
    let output_path = match_result
        .get_one::<String>("output_path")
        .unwrap_or(default_output);

    create_json_file_from_video(video_path, "frames", output_path).await;
    /*  let video_path = &args[1];
    create_json_file_from_video(video_path, "frames", "output.json").await; */
}
