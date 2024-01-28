mod sqlstitch;

use std::{
    env,
    fs::File,
    io::{self, Read},
};

use sqlstitch::{extract_foreign_key_constraints, sort_tables_by_foreign_key_constraints, RelatedTables};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Please provide multiple files to read from.");
        std::process::exit(1);
    }

    let (_, files) = args.split_at(1);

    let mut tables: Vec<RelatedTables> = Vec::new();

    for file in files {
        // get content of file
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
