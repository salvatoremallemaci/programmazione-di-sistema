use minesweeper::annotate;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]    // implementa Parser e Debug
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Rows of the minefield
    #[clap(short, long)]    // short (accetta -p) o long (accetta --par)
    rows: u32,

    /// Columns of the minefield
    #[clap(short, long)]    // short (accetta -p) o long (accetta --par)
    cols: u32,

    /// Minefield
    #[clap(short, long)]    // short (accetta -p) o long (accetta --par)
    minefield: String,
}


fn main() {
}
