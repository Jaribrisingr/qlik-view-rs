use clap::Parser;

#[derive(Parser, Debug)]
#[command(about= "A cli qvd reader")]
pub struct Arguments {
    // Location of the qvd file
    pub path: String,

    // Number of records to show
    #[arg(long, short = 'r', default_value_t = 20)]
    pub max_rows: usize,

    // The character width of each cell
    #[arg(long, short = 'w', default_value_t = 30)]
    pub cell_width: usize,
    
    // Number of columns to show
    #[arg(long, short = 'c', default_value_t = 10)]
    pub max_columns: usize,

    // Return QVD metadata
    #[arg(long, short = 'm', default_value_t = false)]
    pub metadata: bool
}
