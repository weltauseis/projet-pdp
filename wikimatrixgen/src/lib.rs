/*
* Copyright (c) 2024, Kevin Jourdain
*
* SPDX-License-Identifier: BSD-3-Clause
*/
use serde::{Deserialize, Serialize};

// Structures for automatic parsing of Wikipedia API responses with serde_json ------------
// Response for article history request
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryRes {
    pub latest: String,           // API route to get the latest revisions
    pub older: Option<String>,    // If available, API route to get the prior revisions
    pub newer: Option<String>,    // If available, API route to get the following revisions
    pub revisions: Vec<Revision>, // Array of 0-20 revisions
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Revision {
    pub id: u32,                // Revision identifier
    pub timestamp: String,      // Time of the edit in ISO 8601 format
    pub comment: String, // Comment or edit summary written by the editor. For revisions without a comment, the API returns null or "".
    pub delta: Option<i64>, // Number of bytes changed, positive or negative, between a revision and the preceding revision (example: -20). If the preceding revision is unavailable, the API returns null.
    pub source: Option<String>, // Revision content in the format specified by the content_model property
}
