use core::time;
use std::io::Write;
use std::{fs::File, thread::sleep};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    /// name of the wikipedia page
    page: String,
    /// output file
    output: String,
    /// language code of the wikipedia page : en, fr, de, ...
    #[arg(short, long)]
    language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Revision {
    id: u32,
    timestamp: String,
    comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct History {
    revisions: Vec<Revision>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompareResponse {
    diff: Vec<Diff>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Diff {
    r#type: u64,
}

fn main() {
    let cli = Cli::parse();

    let lang = cli.language.unwrap_or("en".to_string());

    let body = reqwest::blocking::get(format!(
        "https://api.wikimedia.org/core/v1/wikipedia/{}/page/{}/history",
        lang, cli.page
    ))
    .unwrap()
    .text()
    .unwrap();

    let history: History = match serde_json::from_str(body.as_str()) {
        Ok(h) => h,
        Err(_) => {
            panic!("Error during request : {}", body);
        }
    };

    let n = history.revisions.len();

    let mut matrix: Vec<f64> = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            println!(
                "Diff between rev #{} and #{}...",
                history.revisions[i].id, history.revisions[j].id
            );

            sleep(time::Duration::from_secs(1));
            let body = reqwest::blocking::get(format!(
                "https://api.wikimedia.org/core/v1/wikipedia/en/revision/{}/compare/{}",
                history.revisions[i].id, history.revisions[j].id
            ))
            .unwrap()
            .text()
            .unwrap();

            let req: Result<CompareResponse, _> = serde_json::from_str(body.as_str());
            let res = match req {
                Ok(c) => c,
                Err(_) => panic!("Error during request : {}", body),
            };

            let mut dist = 0.0;

            for d in res.diff {
                if d.r#type == 0 {
                    dist += 1.0
                };
            }

            matrix.push(dist);
        }
    }

    println!("{:?}", matrix);

    let mut output_file = File::create(cli.output).unwrap();

    writeln!(&mut output_file, "{{").unwrap();
    // distance matrix
    writeln!(&mut output_file, "    \"distancematrix\": [").unwrap();
    for i in 0..n {
        write!(&mut output_file, "        [").unwrap();
        for j in 0..n {
            write!(&mut output_file, "{}", matrix[i * n + j]).unwrap();
            if j < n - 1 {
                write!(&mut output_file, ",").unwrap();
            }
        }
        write!(&mut output_file, "]").unwrap();
        if i < n - 1 {
            write!(&mut output_file, ",").unwrap();
        }
        writeln!(&mut output_file).unwrap();
    }
    writeln!(&mut output_file, "    ],").unwrap();
    // data
    writeln!(&mut output_file, "    \"data\": [").unwrap();
    writeln!(&mut output_file, "        {{").unwrap();
    writeln!(&mut output_file, "            \"name\": \"{}\",", cli.page).unwrap();
    writeln!(&mut output_file, "            \"timelabels\": [").unwrap();
    for j in 0..n {
        write!(
            &mut output_file,
            "                \"{}\"",
            history.revisions[j].timestamp
        )
        .unwrap();
        if j < n - 1 {
            write!(&mut output_file, ",").unwrap();
        }
        writeln!(&mut output_file).unwrap();
    }
    writeln!(&mut output_file, "            ],").unwrap();
    writeln!(&mut output_file, "        }}").unwrap();
    writeln!(&mut output_file, "    ]").unwrap();
    writeln!(&mut output_file, "}}").unwrap();
}
