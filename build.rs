use std::{env, fs, path::Path};

fn collect(root: &Path, dir: &Path, files: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("asset directory") {
        let path = entry.expect("asset entry").path();
        if path.is_dir() {
            collect(root, &path, files);
        } else if path
            .extension()
            .is_some_and(|e| matches!(e.to_str(), Some("png" | "json" | "mid" | "wav" | "xmob")))
        {
            files.push(
                path.strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/"),
            );
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=assets");
    if env::var("CARGO_CFG_TARGET_ARCH").as_deref() != Ok("wasm32") {
        return;
    }
    let root = Path::new("assets").canonicalize().unwrap();
    let mut files = Vec::new();
    collect(&root, &root, &mut files);
    files.sort();
    let mut source = String::from("pub static EMBEDDED: &[(&str, &[u8])] = &[\n");
    for name in files {
        source.push_str(&format!(
            "({name:?}, include_bytes!({:?})),\n",
            root.join(&name)
        ));
    }
    source.push_str("];\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("assets.rs"),
        source,
    )
    .unwrap();
}
