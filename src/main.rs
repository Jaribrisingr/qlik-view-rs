use vqvd::{args::Arguments, qvd_parser::{read_qvd, read_qvd_metadata}};
use clap::Parser;
use std::env;

fn main() {
    let args: Arguments = Arguments::parse();
    configure_the_environment(&args);
    if args.metadata {
        read_qvd_metadata(args)
    } else {
        read_qvd(args)
    }
}

/// Configure Polars with ENV vars
fn configure_the_environment(args: &Arguments) {
    env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "0");
    env::set_var("POLARS_FMT_MAX_COLS", format!("{}", args.max_columns));
    env::set_var("POLARS_FMT_MAX_ROWS",  format!("{}", args.max_rows));
    env::set_var("POLARS_FMT_STR_LEN", format!("{}", args.cell_width));
}

