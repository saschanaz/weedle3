extern crate weedle;

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use nom::error::{convert_error, VerboseError};

fn convert_err(content: &str, err: nom::Err<VerboseError<&str>>) -> String {
    use nom::Err::*;
    match err {
        Error(e) | Failure(e) => convert_error(content, e),
        Incomplete(_) => "Unexpected incomplete error".to_owned(),
    }
}

fn iterate<F>(path: &str, func: F) -> std::io::Result<()>
where
    F: Fn(&DirEntry, Result<Vec<weedle::Definition>, String>) -> std::io::Result<()>,
{
    let read_dir = std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap();

    for entry_result in read_dir {
        let entry = entry_result?;

        let content = std::fs::read_to_string(entry.path()).unwrap();
        let result = weedle::parse(&content).map_err(|err| convert_err(&content, err));

        func(&entry, result)?;
    }

    Ok(())
}

fn get_out_path(out_dir_path: &Path, entry: &DirEntry) -> PathBuf {
    let path = entry.path();
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    out_dir_path.join(format!("{file_stem}.txt"))
}

fn main() -> std::io::Result<()> {
    let out_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/baselines/defs/");
    iterate("tests/defs/", |entry, result| {
        let parsed = result.unwrap_or_else(|err| {
            let file_name = entry.file_name();
            panic!("Failed to parse {file_name:?}:\n{err}");
        });

        let out_file_path = get_out_path(&out_dir_path, entry);
        std::fs::write(out_file_path, format!("{parsed:#?}\n"))?;
        Ok(())
    })?;

    let out_dir_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/baselines/invalids/");
    iterate("tests/invalids/", |entry, result| {
        let err = match result {
            Ok(_) => {
                let path = entry.path();
                panic!("{path:?} unexpected got no error");
            }
            Err(err) => err,
        };

        let out_file_path = get_out_path(&out_dir_path, entry);
        std::fs::write(out_file_path, err)?;
        Ok(())
    })?;

    Ok(())
}
