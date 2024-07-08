use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};

use crate::model::GraphDocument;

impl GraphDocument {
    /// Load Obographs graph document from provided path.
    pub fn from_path<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        GraphDocument::from_reader(BufReader::new(File::open(path).context("Opening file")?))
    }

    /// Load Obographs graph document from a buffered reader.
    pub fn from_reader<R>(read: R) -> Result<Self>
    where
        R: BufRead,
    {
        Ok(serde_json::from_reader(read).context("Reading JSON")?)
    }
}
