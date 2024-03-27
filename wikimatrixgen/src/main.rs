use std::process::exit;

use clap::Parser;
use edit_distance::edit_distance;
use rayon::prelude::*;
use serde_json::json;
use std::io::Write;
use std::sync::Mutex;
use wikimatrixgen::{HistoryRes, Revision};

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
    /// language code of the wikipedia page : en, fr, de, ...
    #[arg(short, default_value = "en")]
    lang_code: String,
    /// Number of latest revisions to take into account
    #[arg(short, default_value = "20")]
    number: usize,
}

// https://www.mediawiki.org/wiki/API:REST_API/Reference
fn main() {
    // parse CLI args
    let cmd = Command::parse();

    // http client
    let client = reqwest::blocking::Client::new();

    let mut revisions = Vec::new();
    let mut url = format!(
        "https://{}.wikipedia.org/w/rest.php/v1/page/{}/history",
        cmd.lang_code, cmd.page
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

    let n = revisions.len();
    let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(n * n);
    matrix.resize(n, vec![0.0; n]);
    let matrix = Mutex::new(matrix);

    println!("Computing distance for every possible pair...");

    for i in 0..n {
        print_progress_bar(i * n - 1, n * n);

        ((i + 1)..n).into_par_iter().for_each(|j| {
            let a = &wikitexts[i];
            let b = &wikitexts[j];
            let distance = edit_distance(a, b) as f64;

            let mut matrix = matrix.lock().unwrap();
            matrix[i][j] = distance;
            drop(matrix); // Unlock the mutex
        });
        print_progress_bar(i * n - 1, n * n);
    }
    println!("");

    for i in 0..n {
        for j in 0..n {
            if j < i {
                let mut matrix = matrix.lock().unwrap();
                matrix[i][j] = matrix[j][i];
            }
        }
    }

    // STEP 4 : WRITE THE JSON FILE

    let json_output = json!({
        "distancematrix": *matrix.lock().unwrap(),
        "data": [{
            "name": cmd.page,
            "timelabels": revisions.iter().map(|rev| rev.timestamp.clone()).collect::<Vec<String>>()
        }]
    });

    let mut f = std::fs::File::create(&cmd.output).unwrap();

    write!(&mut f, "{}", json_output.to_string()).unwrap();

    println!("Done ! File saved in '{}'.", &cmd.output);
}

fn print_progress_bar(i: usize, max: usize) {
    let progress = (i + 1) as f64 / max as f64 * 100.0;
    print!("\rProgress : [");
    for i in (0..100).step_by(3) {
        print!("{}", if i <= progress as i32 { "#" } else { "-" });
    }
    print!("] {:.0}% ({} / {})", progress, i + 1, max);
    std::io::stdout().flush().unwrap();
}
