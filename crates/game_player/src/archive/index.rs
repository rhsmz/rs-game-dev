use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub offset: u64,
    pub size: u64,
    pub compressed: bool,
}

#[derive(Debug, Default)]
pub struct AssetIndex {
    pub entries: HashMap<String, ArchiveEntry>,
}
