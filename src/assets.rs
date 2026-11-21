//! Read-only resources shared by the native filesystem and embedded web build.
use std::{borrow::Cow, path::PathBuf};

pub trait Assets {
    fn read(&self, name: &str) -> Result<Cow<'_, [u8]>, String>;
    /// Full resource names beneath a directory, in stable order.
    fn list(&self, directory: &str) -> Result<Vec<String>, String>;
}

pub struct FileAssets(pub PathBuf);
impl Assets for FileAssets {
    fn read(&self, name: &str) -> Result<Cow<'_, [u8]>, String> {
        std::fs::read(self.0.join(name.trim_start_matches('/')))
            .map(Cow::Owned)
            .map_err(|e| format!("{name}: {e}"))
    }
    fn list(&self, directory: &str) -> Result<Vec<String>, String> {
        let mut names = Vec::new();
        for entry in std::fs::read_dir(self.0.join(directory)).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().is_file() {
                names.push(format!(
                    "{directory}/{}",
                    entry.file_name().to_string_lossy()
                ));
            }
        }
        names.sort();
        Ok(names)
    }
}

#[cfg(target_arch = "wasm32")]
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

#[cfg(target_arch = "wasm32")]
pub struct EmbeddedAssets;
#[cfg(target_arch = "wasm32")]
impl Assets for EmbeddedAssets {
    fn read(&self, name: &str) -> Result<Cow<'_, [u8]>, String> {
        let name = name.trim_start_matches('/');
        EMBEDDED
            .binary_search_by_key(&name, |(key, _)| *key)
            .map(|i| Cow::Borrowed(EMBEDDED[i].1))
            .map_err(|_| format!("Missing resource: {name}"))
    }
    fn list(&self, directory: &str) -> Result<Vec<String>, String> {
        let prefix = format!("{directory}/");
        Ok(EMBEDDED
            .iter()
            .filter(|(name, _)| name.starts_with(&prefix))
            .map(|(name, _)| (*name).to_owned())
            .collect())
    }
}
