pub(crate) mod crc32;

use anyhow::{anyhow, Result};

pub fn time() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

pub fn extract_id_from_filename(entry: &std::path::PathBuf) -> Result<u128> {
    entry
        .extension()
        .ok_or_else(|| anyhow!("Missing extension (ie. not in format: data.<id>)"))?
        .to_str()
        .unwrap()
        .parse()
        .map_err(Into::into)
}
