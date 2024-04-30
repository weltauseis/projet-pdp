/*
* Copyright (c) 2024, Kevin Jourdain
*
* SPDX-License-Identifier: BSD-3-Clause
*/

use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;
use std::io::Write;
use std::{process::exit, sync::Mutex};
use textdistance::str::levenshtein;
use wikimatrixgen::{HistoryRes, Revision};

// struct for automatic cli argument parsing with clap
#[derive(Parser)]
#[command(
    about = "A simple tool to generate distance matrices for time curves visualisation from a wikipedia article."
)]
struct Command {
    /// name of the wikipedia page in URL, e.g. "Hideo_Kojima"
    page: String,
    /// output file
    output: String,
    /// language code of the wikipedia page : en, fr, de, ...
    #[arg(short, long, default_value = "en", value_name = "CODE")]
    lang_code: String,
    /// Number of latest revisions to take into account
    #[arg(short, long, default_value = "20")]
    number: usize,
    /// If specified, include only revisions older than this revision
    #[arg(short, long, value_name = "REVISION_ID")]
    older_than: Option<String>,
    /// Save the timestamps in a format compatible with the original Java implementation
    #[arg(long)]
    legacy: bool,
}

// https://www.mediawiki.org/wiki/API:REST_API/Reference
fn main() {
    // parse CLI args
    let cmd = Command::parse();

    // http client
    let client = reqwest::blocking::Client::new();

    let mut revisions = Vec::new();
    let mut url = format!(
        "https://{}.wikipedia.org/w/rest.php/v1/page/{}/history{}",
        cmd.lang_code,
        cmd.page,
        if let Some(older) = cmd.older_than {
            format!("?older_than={}", older)
        } else {
            String::new()
        }
    );

    // STEP 1 : GET ALL THE REVISIONS

    // fill the revisions array
    println!("Getting a list of revisions from Wikipedia's API...");
    while revisions.len() < cmd.number {
        // call the API to get 20 revisions
        let response_body = client.get(&url).send().unwrap().text().unwrap();

        // parse the response into a HistoryRes object
        let history: HistoryRes = match serde_json::from_str(response_body.as_str()) {
            Ok(h) => h,
            Err(_) => {
                println!("Error during request : {}", response_body);
                exit(1);
            }
        };

        // add all revisions to the array
        revisions.extend(history.revisions);

        // update the URL
        if let Some(new_url) = history.older {
            url = new_url;
        } else {
            break; // no more revisions to fetch
        }
    }

    // if we fetched more than enough, truncate the array to keep only n revisions
    revisions.truncate(cmd.number);

    println!("Done ! {} revisions found.", revisions.len());
    println!(
        "Fist revision is {} at {}.",
        revisions.first().unwrap().id,
        revisions.first().unwrap().timestamp
    );
    println!(
        "Last revision is {} at {}.",
        revisions.last().unwrap().id,
        revisions.last().unwrap().timestamp
    );

    // STEP 2 : FETCH THE WIKITEXT SOURCE CODE FOR EACH REVISION
    println!("Fetching wikitext source for all revisions...");
    let mut wikitexts = Vec::new();
    wikitexts.resize(revisions.len(), String::new());
    for (i, rev) in revisions.iter().enumerate() {
        print_progress_bar(i, revisions.len());
        let response_body = client
            .get(format!(
                "https://{}.wikipedia.org/w/rest.php/v1/revision/{}",
                cmd.lang_code, rev.id
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let revision_with_source: Revision = match serde_json::from_str(response_body.as_str()) {
            Ok(h) => h,
            Err(_) => {
                println!("Error during request : {}", response_body);
                exit(1);
            }
        };

        // we can panic here since there's nothing we can do without the source anyway
        let src = revision_with_source.source.unwrap();
        wikitexts[i] = src;
    }
    println!("");

    // STEP 3 : COMPUTE LEVENSHTEIN DISTANCE BETWEEN EACH PAIR TO PRODUCE A DISTANCE MATRIX
    println!("Computing distance for every possible pair...");

    // nb of rows / cols of matrix
    let n = revisions.len();

    // Create a shared counter and a mutex to protect it
    let counter = Mutex::new(0);

    // compute distance in parallel, for half the matrix
    print_progress_bar(0, n * (n - 1) / 2);
    let mut matrix = (0..n)
        .into_iter()
        .map(|i| /* for every row */ {
            (0..n)
                .into_par_iter()
                .map(|j| /* for every element of the row */ {
                    if j <= i {
                        0.0
                    } else {
                        let a = &wikitexts[i];
                        let b = &wikitexts[j];

                        let dist = levenshtein(a, b) as f64;

                        {
                            let mut c = counter.lock().unwrap();
                            *c += 1;
                            print_progress_bar(*c - 1, n * (n - 1) / 2);
                        }

                        dist
                    }
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
    println!("");

    // mirror the computed half
    for i in 0..n {
        for j in 0..n {
            if j < i {
                matrix[i][j] = matrix[j][i];
            }
        }
    }

    // STEP 4 : WRITE THE JSON FILE
    let json_output = json!({
        "distancematrix": matrix,
        "data": [{
            "name": format!("{}.wikipedia.org/wiki/{}", cmd.lang_code, cmd.page),
            "timelabels": revisions.iter().map(|rev|
                if !cmd.legacy {
                    rev.timestamp.clone()
                } else {
                    rev.timestamp.replace("T", " ").replace("Z", ".0")
                }
            ).collect::<Vec<String>>()
        }]
    });

    let mut f = std::fs::File::create(&cmd.output).unwrap();

    write!(
        &mut f,
        "{}",
        serde_json::to_string_pretty(&json_output).unwrap()
    )
    .unwrap();

    println!("Done ! File saved in '{}'.", &cmd.output);
}

fn print_progress_bar(i: usize, max: usize) {
    let progress = i as f64 / (max - 1) as f64 * 100.0;
    print!("\rProgress : [");
    for i in (0..100).step_by(3) {
        print!("{}", if i <= progress as i32 { "#" } else { "-" });
    }
    print!("] {:.0}% ({} / {})", progress, i + 1, max);
    std::io::stdout().flush().unwrap();
}
