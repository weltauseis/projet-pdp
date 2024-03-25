use videomatrixgen::video_to_frames;

fn main(){
    let video_path = "./test.mp4";
    let output_path = "output";
    video_to_frames(video_path, output_path).unwrap();
}