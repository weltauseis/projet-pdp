//this file will contain the code for the video converter
//video are cut into 1 seconds frames and then converted to images
//the images are converted  to a distance matrix

use image;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread::scope;

pub struct Video {
    pub path: String,
    pub frames: Vec<Frame>,
}

pub struct Frame {
    pub path: String,
    pub timestamp: String,
}

//convert a video to a set of frames
pub fn video_to_frames(video_path: &str, output_path: &str) -> Result<Video, Box<dyn Error>> {
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
        .args(&["-i", video_path, "-vf", "fps=1", &output_path])
        .output()?;
    let mut i = 1;
    loop {
        let frame_path = format!("{}/frame{:04}.png", old_output_path.to_str().unwrap(), i);
        if !Path::new(&frame_path).exists() {
            break;
        }
        let frame = Frame {
            path: frame_path,
            timestamp: format! {"2000-01-01 00:{:02}:{:02}.0", i / 60, i % 60},
        };
        _video.frames.push(frame);
        i += 1;
    }
    Ok(_video)
}
//compute the normalized absolute pixel difference between two frames
//DANP = Σ |I1(x, y) - I2(x, y)| / N

pub fn frame_distance(frame1: &Frame, frame2: &Frame) -> i32 {
    let img1 = image::open(&frame1.path).unwrap();
    let img2 = image::open(&frame2.path).unwrap();
    let img1 = img1.to_luma8();
    let img2 = img2.to_luma8();
    let mut distance = 0;
    for (p1, p2) in img1.pixels().zip(img2.pixels()) {
        let p1 = p1.0[0] as i32;
        let p2 = p2.0[0] as i32;
        distance += (p1 - p2).abs();
    }
    distance /= img1.width() as i32 * img1.height() as i32;
    distance
}




//compute the distance matrix between all frames in a video
pub fn distance_matrix_calculate(video: &Video) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; video.frames.len()]; video.frames.len()];
    let mut _distance = 0;
    for i in 0..video.frames.len() {
        for j in (0+i)..video.frames.len() {
            if i == j {
                continue;
            }
            _distance = frame_distance(&video.frames[i], &video.frames[j]);
            matrix[i][j] = _distance;
            matrix[j][i] = _distance;
        }
    }
    matrix
}

pub fn distance_matrix_calculate_multithreads(video: &Video) -> Vec<Vec<i32>> {
    let matrix = Arc::new(Mutex::new(vec![vec![0; video.frames.len()]; video.frames.len()]));
    let mut _distance = 0;
    scope(|s| {
        for i in 0..video.frames.len() {
            let matrix = Arc::clone(&matrix);
            s.spawn(move || {
                for j in (0+i)..video.frames.len() {
                    if i == j {
                        continue;
                    }
                    _distance = frame_distance(&video.frames[i], &video.frames[j]);
                    let mut matrix = matrix.lock().unwrap();
                    matrix[i][j] = _distance;
                    matrix[j][i] = _distance;
                }
            });
        }
    });
    Arc::try_unwrap(matrix).unwrap().into_inner().unwrap()
}

pub fn create_json_file_from_video(input_video: &str, output_images: &str, output_file: &str) {
    let video = video_to_frames(input_video, output_images).unwrap();
    let distance_matrix = distance_matrix_calculate_multithreads(&video);
    let mut output_file = File::create(output_file).unwrap();

    writeln!(&mut output_file, "{{").unwrap();
    // distance matrix
    writeln!(&mut output_file, "    \"distancematrix\": [").unwrap();
    for i in 0..video.frames.len() {
        write!(&mut output_file, "        [").unwrap();
        for j in 0..video.frames.len() {
            write!(&mut output_file, "{}", distance_matrix[i][j]).unwrap();
            if j < video.frames.len() - 1 {
                write!(&mut output_file, ",").unwrap();
            }
        }
        write!(&mut output_file, "]").unwrap();
        if i < video.frames.len() - 1 {
            write!(&mut output_file, ",").unwrap();
        }
        writeln!(&mut output_file).unwrap();
    }
    writeln!(&mut output_file, "    ],").unwrap();
    // data
    writeln!(&mut output_file, "    \"data\": [").unwrap();
    writeln!(&mut output_file, "        {{").unwrap();
    writeln!(
        &mut output_file,
        "            \"name\": \"{}\",",
        video.path
    )
    .unwrap();
    writeln!(&mut output_file, "            \"timelabels\": [").unwrap();
    for j in 0..video.frames.len() {
        write!(
            &mut output_file,
            "                \"{}\"",
            video.frames[j].timestamp
        )
        .unwrap();
        if j < video.frames.len() - 1 {
            write!(&mut output_file, ",").unwrap();
        }
        writeln!(&mut output_file).unwrap();
    }
    writeln!(&mut output_file, "            ]").unwrap();
    writeln!(&mut output_file, "        }}").unwrap();
    writeln!(&mut output_file, "    ]").unwrap();
    writeln!(&mut output_file, "}}").unwrap();
}
