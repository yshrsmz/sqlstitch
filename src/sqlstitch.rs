use std::collections::HashMap;

use petgraph::{algo::toposort, graph::DiGraph};
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
    let mut name = String::new();

    for statement in ast {
        if let Statement::CreateTable {
            name: table_name,
            constraints,
            ..
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

    sorted_indices
        .iter()
        .rev()
        .map(|i| graph[*i].clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_extract_foreign_key_constraints {
        use super::*;

        #[test]
        fn should_not_contain_related_tables_if_no_constraints() {
            let sql = r#"
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT
            );
            "#;

            let related_tables = extract_foreign_key_constraints(sql.to_string());
            assert_eq!(related_tables.source, sql);
            assert_eq!(related_tables.name, "users");
            assert_eq!(related_tables.related_tables.len(), 0);
        }

        #[test]
        fn should_contain_related_tables() {
            let sql = r#"
            CREATE TABLE posts (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                FOREIGN KEY (user_id) REFERENCES users (id)
            );
            "#;

            let related_tables = extract_foreign_key_constraints(sql.to_string());
            assert_eq!(related_tables.source, sql);
            assert_eq!(related_tables.name, "posts");
            assert_eq!(related_tables.related_tables, vec!["users"]);
        }
    }

    mod sort_tables_by_foreign_key {
        use super::*;

        #[test]
        fn should_sort_by_constraints() {
            let tables = vec![
                RelatedTables {
                    source: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);".to_string(),
                    name: "users".to_string(),
                    related_tables: vec![],
                },
                RelatedTables {
                    source: "CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, FOREIGN KEY (user_id) REFERENCES users (id));".to_string(),
                    name: "posts".to_string(),
                    related_tables: vec!["users".to_string()],
                },
            ];

            let sorted_tables = sort_tables_by_foreign_key_constraints(&tables);
            assert_eq!(sorted_tables, vec!["users", "posts"]);
        }
    }
}
