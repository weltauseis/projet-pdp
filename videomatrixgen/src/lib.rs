//this file will contain the code for the video converter
//video are cut into 1 seconds frames and then converted to images
//the images are converted  to a distance matrix

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use image;

pub struct Video {
    path: String,
    frames: Vec<Frame>,
}

pub struct Frame {
    path: String,
    timestamp: String,
}

//convert a video to a set of frames
pub fn video_to_frames(video_path: &str, output_path: &str) -> Result<(), Box<dyn Error>>{
    let old_output_path = Path::new(output_path);
    let mut _video = Video {
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
    let mut i = 1;
    loop {
        let frame_path = format!("{}/frame{:04}.png", old_output_path.to_str().unwrap(), i);
        if !Path::new(&frame_path).exists() {
            break;
        }
        let frame = Frame {
            path: frame_path,
            timestamp: i.to_string(),
        };
        _video.frames.push(frame);
        i += 1;
    }
    Ok(())
}
//compute the normalized absolute pixel difference between two frames
//DANP = Σ |I1(x, y) - I2(x, y)| / N
