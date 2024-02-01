use std::collections::HashMap;

use petgraph::{graph::DiGraph, algo::toposort};
use sqlparser::{ast::Statement, dialect, parser::Parser};

#[derive(Clone, Debug)]
pub struct RelatedTables {
    pub source: String,
    pub name: String,
    pub related_tables: Vec<String>,
}

pub fn extract_foreign_key_constraints(sql: String) -> RelatedTables {
    let dialect = dialect::GenericDialect {};
    let ast_result = Parser::parse_sql(&dialect, &sql);
    let ast = match ast_result {
        Ok(ast) => ast,
        Err(e) => {
            panic!("Error parsing SQL: {}", e);
        }
    };

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
                    foreign_table.0.iter().for_each(|s| {
                        related_tables.push(s.value.to_string());
                    });
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

pub fn sort_tables_by_foreign_key_constraints(tables: &Vec<RelatedTables>) -> Vec<String> {
    let mut graph = DiGraph::<_, ()>::new();
    let mut indices = HashMap::new();

    for table in tables {
        indices.insert(table.name.clone(), graph.add_node(table.name.clone()));
    }

    for table in tables {
        let source_index = *indices.get(&table.name).unwrap();
        for related_table in &table.related_tables {
            let target_index = *indices.get(related_table).unwrap();
            graph.add_edge(source_index, target_index, ());
        }
    }

    let sorted_indices = toposort(&graph, None).unwrap();

    sorted_indices.iter().rev().map(|i| graph[*i].clone()).collect()
}
