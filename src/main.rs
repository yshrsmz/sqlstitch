use std::{
    env,
    fs::File,
    io::{self, Read},
};

use petgraph::{graph::DiGraph, algo::toposort};
use sqlparser::{ast::Statement, dialect, parser::Parser};

#[derive(Clone, Debug)]
struct RelatedTables {
    source: String,
    name: String,
    related_tables: Vec<String>,
}

fn extract_foreign_key_constraints(sql: String) -> RelatedTables {
    let dialect = dialect::GenericDialect {};
    let ast = Parser::parse_sql(&dialect, &sql).unwrap();

    let mut related_tables = Vec::new();
    let mut name =String::new();

    for statement in ast {
        if let Statement::CreateTable {
            name: table_name, constraints, ..
        } = statement
        {
            name = table_name.to_string();
            for constraint in constraints {
                if let sqlparser::ast::TableConstraint::ForeignKey { foreign_table, .. } =
                    constraint
                {
                    related_tables.push(foreign_table.to_string());
                }
            }
        }
    }

    RelatedTables {
        source: sql,
        name,
        related_tables,
    }
}

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

    let mut graph = DiGraph::<_, ()>::new();
    let mut indices = std::collections::HashMap::new();

    for table in &tables {
        indices.insert(table.name.clone(), graph.add_node(table.name.clone()));
    }

    for table in &tables {
        let source_index = *indices.get(&table.name).unwrap();
        for related_table in &table.related_tables {
            let target_index = *indices.get(related_table).unwrap();
            graph.add_edge(source_index, target_index, ());
        }
    }

    let sorted_indices = toposort(&graph, None).unwrap();
    let sorted_tables: Vec<_> = sorted_indices.iter().rev().map(|i| graph[*i].clone()).collect();

    for table in sorted_tables {
        let result = tables.iter().find(|t| t.name == table).unwrap();
        println!("{}", result.source);
    }

    Ok(())
}
