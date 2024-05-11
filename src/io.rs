use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::model::GraphDocument;

#[cfg(feature = "serde")]
impl GraphDocument {
    /// Load Obographs graph document from provided path.
    pub fn from_path<T>(path: T) -> Result<Self, Box<dyn Error>>
    where
        T: AsRef<Path>,
    {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        GraphDocument::from_reader(&mut reader)
    }

    /// Load Obographs graph document from a buffered reader.
    pub fn from_reader<T>(read: &mut T) -> Result<Self, Box<dyn Error>>
    where
        T: BufRead,
    {
        Ok(serde_json::from_reader(read)?)
    }
}
