use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

/// Encoded records, including the ZMR1 envelope. Missing records return None.
pub trait RecordStore {
    fn read(&self, name: &str) -> Result<Option<Vec<u8>>, String>;
    fn write(&self, name: &str, bytes: &[u8]) -> Result<(), String>;
    fn delete(&self, name: &str) -> Result<(), String>;
    fn list(&self) -> Result<Vec<String>, String>;
}

pub struct FileRecords(pub PathBuf);
impl RecordStore for FileRecords {
    fn read(&self, name: &str) -> Result<Option<Vec<u8>>, String> {
        match fs::read(self.0.join(format!("{name}.rms"))) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
    fn write(&self, name: &str, bytes: &[u8]) -> Result<(), String> {
        atomic_write(&self.0.join(format!("{name}.rms")), bytes).map_err(|e| e.to_string())
    }
    fn delete(&self, name: &str) -> Result<(), String> {
        fs::remove_file(self.0.join(format!("{name}.rms"))).map_err(|e| e.to_string())
    }
    fn list(&self) -> Result<Vec<String>, String> {
        let entries = match fs::read_dir(&self.0) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.to_string()),
        };
        let mut names = entries
            .filter_map(Result::ok)
            .filter_map(|e| {
                e.file_name()
                    .to_str()?
                    .strip_suffix(".rms")
                    .map(str::to_owned)
            })
            .collect::<Vec<_>>();
        names.sort();
        Ok(names)
    }
}
pub fn data_dir() -> PathBuf {
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("Library/Application Support/Zelda Mobile")
}
pub fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension(format!("tmp-{}", std::process::id()));
    let mut file = File::create(&temp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(temp, path)
}
pub fn encode_record(payload: &[u8]) -> Vec<u8> {
    let mut b = b"ZMR1".to_vec();
    b.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    b.extend_from_slice(payload);
    b
}
pub fn decode_record(bytes: &[u8]) -> Result<Vec<u8>, String> {
    if bytes.len() < 8 || &bytes[..4] != b"ZMR1" {
        return Err("Unsupported or damaged save file".into());
    }
    let len = u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize;
    if len != bytes.len() - 8 {
        return Err("Truncated save file".into());
    }
    Ok(bytes[8..].to_vec())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_and_reject_truncation() {
        let data = (0..255).collect::<Vec<u8>>();
        assert_eq!(decode_record(&encode_record(&data)).unwrap(), data);
        assert!(decode_record(&encode_record(&data)[..200]).is_err());
    }
}
