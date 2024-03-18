use serde::{Deserialize, Serialize};

// Structures for automatic parsing of Wikipedia API responses with serde_json ------------
// Response for article history request
#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryRes {
    pub revisions: Vec<Revision>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Revision {
    pub id: u32,
    pub timestamp: String,
    pub comment: String,
}
// Response for article revisions comparison request
#[derive(Serialize, Deserialize, Debug)]
pub struct CompareRes {
    pub diff: Vec<Diff>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Diff {
    pub r#type: u64,
}
