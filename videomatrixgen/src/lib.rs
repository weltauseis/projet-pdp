//this file will contain the code for the video converter
//video are cut into 1 seconds frames and then converted to images
//the images are converted via a color histogram comparaison to a distance matrix

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Video {
    path: String,
    frames: Vec<Frame>,
}

pub struct Frame {
    path: String,
    timestamp: String,
    histogram: Vec<u32>,
}

//convert a video to a set of frames
pub fn video_to_frames(video_path: &str, output_path: &str) -> Result<(), Box<dyn Error>>{
    let _video = Video {
        path: video_path.to_string(),
        frames: Vec::new(),
    };

    //create the output directory if it does not exist
    let output_path = Path::new(output_path);
    if !output_path.exists() {
        fs::create_dir(output_path)?;
    }

    let output_path = output_path.to_str().unwrap();
    let output_path = format!("{}/frame%04d.png", output_path);
    Command::new("ffmpeg")
        .args(&[
            "-i",
            video_path,
            "-vf",
            "fps=1",
            &output_path,
        ])
        .output()?;
    Ok(())
}



