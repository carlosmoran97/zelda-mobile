//! Strict decoder used to audit the original XMOB resource catalog.
use std::path::Path;
#[derive(Debug)]
pub struct Map {
    pub width: u8,
    pub height: u8,
    pub tileset: u8,
    pub tiles: Vec<u8>,
    pub warps: Vec<[u8; 6]>,
    pub enemies: Vec<[u8; 3]>,
    pub items: Vec<[u8; 4]>,
    pub objects: Vec<[u8; 10]>,
    pub text: Vec<String>,
}
impl Map {
    pub fn decode(b: &[u8]) -> Result<Self, String> {
        let mut p = 0;
        fn take<'a>(b: &'a [u8], p: &mut usize, n: usize) -> Result<&'a [u8], String> {
            let end = p.checked_add(n).ok_or("map size overflow")?;
            let s = b
                .get(*p..end)
                .ok_or_else(|| format!("Truncated XMOB at offset {}", *p))?;
            *p = end;
            Ok(s)
        }
        fn records<const N: usize>(b: &[u8], p: &mut usize) -> Result<Vec<[u8; N]>, String> {
            let count = take(b, p, 1)?[0];
            (0..count)
                .map(|_| Ok(take(b, p, N)?.try_into().unwrap()))
                .collect()
        }
        if take(b, &mut p, 4)? != b"XMOB" {
            return Err("Invalid XMOB signature".into());
        }
        let head = take(b, &mut p, 3)?;
        let (width, height, tileset) = (head[0], head[1], head[2]);
        if width == 0 || height == 0 {
            return Err("Empty XMOB grid".into());
        }
        let tiles = take(b, &mut p, width as usize * height as usize)?.to_vec();
        let warps = records(b, &mut p)?;
        let enemies = records(b, &mut p)?;
        let items = records(b, &mut p)?;
        let objects = records(b, &mut p)?;
        let mut text = Vec::new();
        let count = take(b, &mut p, 1)?[0];
        for _ in 0..count {
            let n = take(b, &mut p, 1)?[0] as usize;
            text.push(take(b, &mut p, n)?.iter().map(|b| char::from(*b)).collect());
        }
        if p != b.len() {
            return Err(format!("{} trailing XMOB bytes", b.len() - p));
        }
        Ok(Self {
            width,
            height,
            tileset,
            tiles,
            warps,
            enemies,
            items,
            objects,
            text,
        })
    }
}
pub fn audit(assets: &Path) -> Result<String, String> {
    let mut count = 0;
    let mut warps = 0;
    let mut enemies = 0;
    let mut items = 0;
    let mut objects = 0;
    let mut texts = 0;
    let mut sets = std::collections::HashSet::new();
    for e in std::fs::read_dir(assets.join("Maps")).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();
        let map = Map::decode(&std::fs::read(&p).map_err(|e| e.to_string())?)
            .map_err(|e| format!("{}: {e}", p.display()))?;
        if (map.width, map.height) != (16, 11) || map.tiles.len() != 176 {
            return Err("Unexpected map dimensions".into());
        }
        count += 1;
        warps += map.warps.len();
        enemies += map.enemies.len();
        items += map.items.len();
        objects += map.objects.len();
        texts += map.text.len();
        sets.insert(map.tileset);
    }
    if count != 505 {
        return Err(format!("Expected 505 maps; found {count}"));
    }
    let mut images = 0;
    for e in std::fs::read_dir(assets.join("Images")).map_err(|e| e.to_string())? {
        let p = e.map_err(|e| e.to_string())?.path();
        crate::graphics::Bitmap::load(&p)?;
        images += 1;
    }
    if images != 240 {
        return Err(format!("Expected 240 images; found {images}"));
    }
    Ok(format!(
        "{count} maps, {} tilesets, {warps} warps, {enemies} enemies, {items} items, {objects} objects, {texts} text entries; {images} PNGs decoded",
        sets.len()
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_resources() {
        assert!(
            audit(&std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")).is_ok()
        );
    }
    #[test]
    fn rejects_truncated_map() {
        let b = std::fs::read("assets/Maps/Map120-0.xmap.xmob").unwrap();
        for n in [0, 4, 6, 100, b.len() - 1] {
            assert!(Map::decode(&b[..n]).is_err());
        }
    }
}
