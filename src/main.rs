use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    path: PathBuf,

    /// Print schemas.
    #[clap(short, long)]
    schema: bool,

    /// Do not format SQL of schemas.
    #[clap(short, long)]
    no_fmt: bool,

    /// Pretty-format SQL of schemas.
    #[clap(short, long)]
    pretty: bool,

    /// Separator to print after each schema, if schema was requested.
    #[clap(long = "sep", default_value = "\n")]
    batch_sep: String,
}

fn main() {
    let cli = Cli::parse();
    let opt = find_sqlite::Options {
        show_schema: cli.schema,
        batch_separator: cli.batch_sep.to_string(),
        format_sql: !cli.no_fmt,
        format_sql_pretty: cli.pretty,
    };
    find_sqlite::run(&cli.path, opt);
}
