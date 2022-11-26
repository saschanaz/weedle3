extern crate weedle;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let read_dir =
        std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/defs/")).unwrap();

    let out_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/baselines/");

    for entry_result in read_dir {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let path = entry.path();
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let out_file_path = out_dir_path.join(format!("{file_stem}.txt"));

        let content = std::fs::read_to_string(entry.path()).unwrap();

        // XXX: Can't be used without unwrap()?
        let parsed =
            weedle::parse(&content).unwrap_or_else(|_| panic!("failed to parse {file_name:?}"));
        std::fs::write(&out_file_path, format!("{parsed:#?}\n"))
            .unwrap_or_else(|_| panic!("Couldn't write to {out_file_path:?}"))
    }

    Ok(())
}
