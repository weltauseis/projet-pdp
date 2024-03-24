use std::fs::File;
use std::io::Write;

use clap::Parser;
use reqwest::header::USER_AGENT;
use wikimatrixgen::{CompareRes, HistoryRes};

// struct for automatic cli argument parsing with clap
#[derive(Parser)]
#[command(
    about = "A simple tool to generate distance matrices for time curves visualisation from a wikipedia article.\n
    NOTE : The wikimedia API only allows for 5000 requests / hour even with a valid token, so this tool only takes\n
     into account the 50 last revisions of an article."
)]
struct Command {
    /// name of the wikipedia page
    page: String,
    /// output file
    output: String,
    /// wikimedia API auth token, see https://api.wikimedia.org/wiki/Getting_started_with_Wikimedia_APIs
    #[arg(short, long)]
    token: Option<String>,
    /// language code of the wikipedia page : en, fr, de, ...
    #[arg(short, long)]
    language: Option<String>,
}

fn main() {
    let cmd = Command::parse();

    let lang_code = cmd.language.unwrap_or("en".to_string());
    let mut token = String::new();

    if let None = cmd.token {
        println!("Enter your wikipedia API token (see https://api.wikimedia.org/wiki/Getting_started_with_Wikimedia_APIs) :");
        std::io::stdin().read_line(&mut token).unwrap();
    } else {
        token = cmd.token.unwrap().clone();
    }

    let client = reqwest::blocking::Client::new();

    println!("Calling wikipedia API to get a list of the article's revisions...");
    let response_body = client
        .get(format!(
            "https://api.wikimedia.org/core/v1/wikipedia/{}/page/{}/history",
            lang_code, cmd.page
        ))
        .header(USER_AGENT, "wikipedia matrix generator")
        .bearer_auth(&token.trim())
        .send()
        .unwrap()
        .text()
        .unwrap();

    let history: HistoryRes = match serde_json::from_str(response_body.as_str()) {
        Ok(h) => h,
        Err(_) => {
            panic!("Error during request : {}", response_body);
        }
    };

    let n = history.revisions.len().clamp(0, 50);
    let mut matrix: Vec<f64> = Vec::with_capacity(n * n);

    println!("Comparing all the revisions...");
    for i in 0..n {
        for j in 0..n {
            let progress = (((i * n + j) as f64 / (n * n) as f64) * 100.0) as u64 + 1;
            print!("\r Progress : [");
            for i in (0..100).step_by(10) {
                print!("{}", if i < progress { "#" } else { "-" });
            }
            print!(
                "] {}% ({}/{}) : Diff between #{} & #{}    ",
                progress,
                i * n + j,
                n * n,
                history.revisions[i].id,
                history.revisions[j].id
            );
            std::io::stdout().flush().unwrap();

            let body = client
                .get(format!(
                    "https://api.wikimedia.org/core/v1/wikipedia/{}/revision/{}/compare/{}",
                    lang_code, history.revisions[i].id, history.revisions[j].id
                ))
                .bearer_auth(&token.trim())
                .header(USER_AGENT, "wikipedia matrix generator")
                .send()
                .unwrap()
                .text()
                .unwrap();

            let req: Result<CompareRes, _> = serde_json::from_str(body.as_str());
            let res = match req {
                Ok(c) => c,
                Err(_) => panic!("Error during request : {}", body),
            };

            let mut dist = 0.0;

            for d in res.diff {
                if d.r#type != 0 {
                    dist += 1.0
                };
            }

            matrix.push(dist);
        }
    }

    let mut output_file = File::create(&cmd.output).unwrap();

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
    writeln!(&mut output_file, "            \"name\": \"{}\",", cmd.page).unwrap();
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
    writeln!(&mut output_file, "            ]").unwrap();
    writeln!(&mut output_file, "        }}").unwrap();
    writeln!(&mut output_file, "    ]").unwrap();
    writeln!(&mut output_file, "}}").unwrap();

    println!("\n Done ! Output file written in {}", &cmd.output);
}
