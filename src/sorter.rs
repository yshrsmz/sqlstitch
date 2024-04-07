use std::collections::HashMap;

use petgraph::{algo::toposort, graph::DiGraph};

use crate::extractor::StatementRelations;

pub fn sort_statements_by_relations(statements: &Vec<StatementRelations>) -> Result<Vec<StatementRelations>, String> {
    let mut graph = DiGraph::<_, ()>::new();
    let mut indices = HashMap::new();

    for statement in statements {
        indices.insert(statement.name.clone(), graph.add_node(statement.clone()));
    }

    for statement in statements {
        let source_index = *indices.get(&statement.name).unwrap();
        for related_statement in &statement.related_statements {
            let target_index = *indices.get(related_statement).unwrap();
            graph.add_edge(source_index, target_index, ());
        }
    }

    let sorted_indices = toposort(&graph, None).unwrap();

    let sorted_statements = sorted_indices
        .iter()
        .rev()
        .map(|i| graph[*i].clone())
        .collect();

    Ok(sorted_statements)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sort_statements_by_relations {
        use super::*;

        #[test]
        fn should_sort_statements_by_relations() {
            let statements = vec![
                StatementRelations {
                    statement: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)".to_string(),
                    name: "users".to_string(),
                    related_statements: Vec::new(),
                },
                StatementRelations {
                    statement: "CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, FOREIGN KEY(user_id) REFERENCES users(id))".to_string(),
                    name: "posts".to_string(),
                    related_statements: vec!["users".to_string()],
                },
            ];

            let result = sort_statements_by_relations(&statements);
            assert!(result.is_ok());
            let sorted_statements = result.unwrap();
            assert_eq!(sorted_statements[0].name, "users");
            assert_eq!(sorted_statements[1].name, "posts");
        }
    }
}
