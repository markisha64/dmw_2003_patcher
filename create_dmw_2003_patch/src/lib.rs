use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Patch {
    pub target: PathBuf,
    pub patch: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatchJSON {
    pub changes: Vec<Patch>,
}
