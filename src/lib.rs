use std::{
    fs::{self, File, Metadata},
    io::{ErrorKind, Read},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

use anyhow::anyhow;
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

const INDENT: &str = "    ";

pub struct Options {
    pub show_metadata: bool,
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
        .filter_map(|path| {
            fs::metadata(&path)
                .inspect_err(|error| {
                    tracing::warn!(
                        ?error,
                        ?path,
                        "Failed to fetch file metadata."
                    );
                })
                .ok()
                .map(|meta| (path, meta))
        })
        .filter(|(_path, meta)| meta.is_file())
        .filter_map(|(path, meta)| {
            file_has_sqlite_header(&path)
                .inspect_err(|error| {
                    tracing::warn!(
                        ?error,
                        ?path,
                        "Failed to check file for SQLite header."
                    );
                })
                .ok()
                .and_then(|has_header| has_header.then_some((path, meta)))
        })
        .filter_map(|(path, meta)| {
            file_fetch_schema(&path, opt.format_sql, opt.format_sql_pretty)
                .inspect_err(|error| {
                    tracing::warn!(?error, ?path, "Failed to fetch schema.");
                })
                .ok()
                .map(|schema| (path, meta, schema))
        })
        .filter_map(|(path, meta, schema)| {
            metadata_fmt(&meta)
                .inspect_err(|error| {
                    tracing::warn!(
                        ?error,
                        ?path,
                        "Failed to format metadata."
                    );
                })
                .ok()
                .map(|meta| (path, meta, schema))
        })
        .for_each(|(path, meta, schema)| {
            let meta = if opt.show_metadata {
                format!("\n{INDENT}meta\n{meta}")
            } else {
                String::new()
            };
            let schema = if opt.show_schema {
                format!("\n{INDENT}schema\n{schema}")
            } else {
                String::new()
            };
            let batch_sep = if opt.show_metadata || opt.show_schema {
                opt.batch_separator.as_str()
            } else {
                ""
            };
            // XXX Single print statement to avoid interleaved output.
            println!("{path:?}{meta}{schema}{batch_sep}");
        });
}

pub fn tracing_init(level: Option<tracing::Level>) -> anyhow::Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

    if let Some(level) = level {
        let layer_stderr = fmt::Layer::new()
            .with_writer(std::io::stderr)
            .with_ansi(true)
            .with_file(false)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_filter(
                EnvFilter::from_default_env().add_directive(level.into()),
            );
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(layer_stderr),
        )?;
    }
    Ok(())
}

fn file_has_sqlite_header(path: &Path) -> anyhow::Result<bool> {
    const SQLITE_HEADER: &[u8; 16] = b"SQLite format 3\0";

    let mut file = File::open(path)?;
    let mut buf = [0u8; SQLITE_HEADER.len()];
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
    Ok(buf[..].eq(SQLITE_HEADER))
}

fn metadata_fmt(meta: &Metadata) -> anyhow::Result<String> {
    let user = meta.uid();
    let group = meta.gid();
    let size = human_units::Size(meta.len());
    let perm = umask::Mode::from(meta.permissions().mode());
    let mtime = humantime::format_rfc3339(meta.modified()?);
    let atime = humantime::format_rfc3339(meta.accessed()?);
    let btime = humantime::format_rfc3339(meta.created()?);
    let lines = [
        format!("btime {btime}"),
        format!("mtime {mtime}"),
        format!("atime {atime}"),
        format!("size {size}"),
        format!("perm {perm}"),
        format!("owner {user}:{group}"),
    ];
    let meta = lines
        .iter()
        .map(|line| format!("{INDENT}{INDENT}{line}"))
        .collect::<Vec<String>>()
        .join("\n");
    Ok(meta)
}

fn file_fetch_schema(
    path: &Path,
    format_sql: bool,
    format_sql_pretty: bool,
) -> anyhow::Result<String> {
    let conn = rusqlite::Connection::open(path)?;
    let sql = "SELECT sql FROM sqlite_master WHERE type IN ('table', 'view', 'index')";
    let mut statement = conn.prepare(sql)?;
    let mut schema = Vec::new();
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        match row.get::<_, String>(0) {
            Err(error) => {
                tracing::warn!(?error, ?row, "Failed to access a row.");
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
                let sql = sql
                    .lines()
                    .map(|line| format!("{INDENT}{INDENT}{line}"))
                    .collect::<Vec<String>>()
                    .join("\n");
                schema.push(sql);
            }
        }
    }
    schema.sort();
    let schema = schema.join("\n");
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
