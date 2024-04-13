//this file will contain the code for the video converter
//video are cut into 1 seconds frames and then converted to images
//the images are converted  to a distance matrix

use image;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tokio::task;

pub struct Video {
    pub path: String,
    pub frames: Vec<Frame>,
}

#[derive(Clone)]
pub struct Frame {
    pub path: String,
    pub timestamp: String,
}

//convert a video to a set of frames
pub fn video_to_frames(
    video_path: &str,
    output_path: &str,
    frame_nb: &u32,
) -> Result<Video, Box<dyn Error>> {
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
    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
        fs::create_dir(output_path)?;
    }

    let output_path = output_path.to_str().unwrap();
    let output_path = format!("{}/frame%04d.png", output_path);
    let frame_str = format!("fps={}", frame_nb);
    Command::new("ffmpeg")
        .args(&["-i", video_path, "-vf", &frame_str, &output_path])
        .output()?;
    //récupère le nombre de frames
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
pub async fn distance_matrix_calculate_multithreads(video: &Video) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; video.frames.len()]; video.frames.len()];
    let mut _distance = 0;
    let mut tasks = Vec::new();
    for i in 0..video.frames.len() {
        for j in (0 + i)..video.frames.len() {
            if i == j {
                continue;
            }
            let frame_i = video.frames[i].clone();
            let frame_j = video.frames[j].clone();
            tasks.push(task::spawn(async move {
                let distance = frame_distance(&frame_i, &frame_j);
                (i, j, distance)
            }));
        }
    }
    for task in tasks {
        let (i, j, distance) = task.await.unwrap();
        matrix[i][j] = distance;
        matrix[j][i] = distance;
    }
    matrix
}

pub async fn create_json_file_from_video(
    input_video: &str,
    output_images: &str,
    output_file: &str,
    frame_nb: &u32,
) {
    let video = video_to_frames(input_video, output_images, frame_nb).unwrap();
    let distance_matrix = (distance_matrix_calculate_multithreads(&video)).await;
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
