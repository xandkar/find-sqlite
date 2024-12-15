use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    path: PathBuf,

    /// Print schemas.
    #[clap(short, long)]
    schema: bool,

    /// Separator to print after each schema, if schema was requested.
    #[clap(long = "sep", default_value = "\n")]
    batch_sep: String,
}

fn main() {
    let cli = Cli::parse();
    find_sqlite::run(&cli.path, cli.schema, &cli.batch_sep);
}
