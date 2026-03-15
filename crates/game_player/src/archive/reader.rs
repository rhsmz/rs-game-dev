use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use thiserror::Error;

use super::index::AssetIndex;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Asset not found: {0}")]
    NotFound(String),
}

pub struct AssetArchive {
    file: BufReader<File>,
    index: AssetIndex,
}

impl AssetArchive {
    pub fn open(path: &str) -> Result<Self, ArchiveError> {
        let file = File::open(path)?;
        let file = BufReader::new(file);

        // Mock index for now, as format is not yet finalized
        let index = AssetIndex::default();

        Ok(Self { file, index })
    }

    pub fn read_asset(&mut self, path: &str) -> Result<Vec<u8>, ArchiveError> {
        let entry = self.index.entries.get(path)
            .ok_or_else(|| ArchiveError::NotFound(path.to_string()))?;

        self.file.seek(SeekFrom::Start(entry.offset))?;
        let mut buffer = vec![0; entry.size as usize];
        self.file.read_exact(&mut buffer)?;

        // Decompression mock (if entry.compressed) would go here

        Ok(buffer)
    }
}
