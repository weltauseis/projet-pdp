use videomatrixgen::create_json_file_from_video;

fn main() {
    let video_path = "./test.mp4";
    let output_path = "output";
    create_json_file_from_video(video_path, output_path, "output.json");
}
