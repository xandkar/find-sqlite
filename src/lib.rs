use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::Path,
};

use anyhow::anyhow;
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

pub fn run(path: &Path, show_schema: bool, batch_sep: &str) {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|entry_result| entry_result.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            file_has_sqlite_header(&path)
                .ok()
                .and_then(|has_header| has_header.then_some(path))
        })
        .filter_map(|path| {
            file_fetch_schema(&path).ok().map(|schema| (path, schema))
        })
        .for_each(|(path, schema)| {
            let (schema, batch_sep) = if show_schema {
                (format!("\n{}", schema.join("\n")), batch_sep)
            } else {
                (String::new(), "")
            };
            // XXX Single print statement to avoid interleaved output.
            println!("{path:?}{schema}{batch_sep}");
        });
}

fn file_has_sqlite_header(path: &Path) -> anyhow::Result<bool> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 16];
    let read_result = file.read_exact(&mut buf);
    match read_result.map_err(|e| e.kind()) {
        Err(ErrorKind::UnexpectedEof) => {
            return Ok(false);
        }
        Err(e) => {
            return Err(anyhow!("{e:?} path={path:?}"));
        }
        Ok(()) => {}
    }
    Ok(buf[..].eq(b"SQLite format 3\0"))
}

fn file_fetch_schema(path: &Path) -> anyhow::Result<Vec<String>> {
    let conn = rusqlite::Connection::open(path)?;
    let sql = "SELECT sql FROM sqlite_master WHERE type IN ('table', 'view', 'index')";
    let mut statement = conn.prepare(sql)?;
    let mut schema = statement
        .query_map([], |row| row.get::<_, String>(0))?
        .filter_map(|result| result.ok())
        .collect::<Vec<String>>();
    schema.sort();
    Ok(schema)
}
