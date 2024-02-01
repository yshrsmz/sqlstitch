mod sqlstitch;
mod commands;

use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use commands::Cli;
use sqlstitch::{extract_foreign_key_constraints, sort_tables_by_foreign_key_constraints, RelatedTables};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut tables: Vec<RelatedTables> = Vec::new();

    for file in cli.files {
        // get content of file
        if cli.verbose {
            println!("Reading file: {}", file);
        }
        let mut content = String::new();
        let mut file = File::open(file)?;
        file.read_to_string(&mut content)?;

        let result = extract_foreign_key_constraints(content);

        tables.push(result);
    }

    let sorted_tables = sort_tables_by_foreign_key_constraints(&tables);

    for table in sorted_tables {
        let result = tables.iter().find(|t| t.name == table).unwrap();
        println!("{}", result.source);
    }

    Ok(())
}
