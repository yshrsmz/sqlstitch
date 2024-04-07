mod extractor;
mod sorter;
mod commands;

use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use commands::Cli;
use extractor::{extract_statements_and_relations, StatementRelations};
use sorter::sort_statements_by_relations;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut statements: Vec<StatementRelations> = Vec::new();

    for file in cli.files {
        // get content of file
        if cli.verbose {
            println!("Reading file: {}", file);
        }
        let mut content = String::new();
        let mut file = File::open(file)?;
        file.read_to_string(&mut content)?;

        let stmts = extract_statements_and_relations(content.clone(), cli.verbose);
        statements.extend(stmts);
    }

    let result = sort_statements_by_relations(&statements);

    if result.is_err() {
        panic!("{}", result.err().unwrap());
    }

    for statement in result.unwrap() {
        println!("{}", statement.statement);
    }

    Ok(())
}
