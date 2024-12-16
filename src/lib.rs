use std::{
    fs::File,
    io::{ErrorKind, Read},
    path::Path,
};

use anyhow::anyhow;
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

pub struct Options {
    pub show_schema: bool,
    pub batch_separator: String,
    pub format_sql: bool,
    pub format_sql_pretty: bool,
}

pub fn run(path: &Path, opt: Options) {
    WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|entry_result| entry_result.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            file_has_sqlite_header(&path)
                .inspect_err(|_error| {
                    // TODO Log errors.
                })
                .ok()
                .and_then(|has_header| has_header.then_some(path))
        })
        .filter_map(|path| {
            file_fetch_schema(&path, opt.format_sql, opt.format_sql_pretty)
                .inspect_err(|_error| {
                    // TODO Log errors.
                })
                .ok()
                .map(|schema| (path, schema))
        })
        .for_each(|(path, schema)| {
            let (schema, batch_sep) = if opt.show_schema {
                (
                    format!("\n{}", schema.join("\n")),
                    opt.batch_separator.as_str(),
                )
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

fn file_fetch_schema(
    path: &Path,
    format_sql: bool,
    format_sql_pretty: bool,
) -> anyhow::Result<Vec<String>> {
    let conn = rusqlite::Connection::open(path)?;
    let sql = "SELECT sql FROM sqlite_master WHERE type IN ('table', 'view', 'index')";
    let mut statement = conn.prepare(sql)?;
    let mut schema = Vec::new();
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        match row.get::<_, String>(0) {
            Err(_error) => {
                // TODO Optional logging.
            }
            Ok(sql) => {
                let sql = if format_sql {
                    if format_sql_pretty {
                        sql_fmt_pretty(&sql)
                    } else {
                        sql_fmt(&sql)
                    }
                } else {
                    sql
                };
                schema.push(sql);
            }
        }
    }
    schema.sort();
    Ok(schema)
}

/// Normalize format - remove inconsistent spaces and newlines.
fn sql_fmt(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn sql_fmt_pretty(sql: &str) -> String {
    use sqlformat::{FormatOptions, Indent, QueryParams};

    let mut opt = FormatOptions::default();
    opt.indent = Indent::Spaces(4);
    opt.uppercase = Some(true);
    sqlformat::format(&sql, &QueryParams::None, &opt)
}
