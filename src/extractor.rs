use sqlparser::{
    ast::{
        CreateIndex, CreateTable,
        Statement,
    },
    dialect,
    parser::Parser,
};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct StatementRelations {
    pub statement: String,
    pub name: String,
    pub related_statements: Vec<String>,
}

pub fn extract_statements_and_relations(sql: String, verbose: bool) -> Vec<StatementRelations> {
    // extract ddls from sql string
    let dialect = dialect::GenericDialect {};
    let ast_result = Parser::parse_sql(&dialect, &sql);
    let ast = match ast_result {
        Ok(ast) => ast,
        Err(e) => {
            panic!("Error parsing SQL: {}", e);
        }
    };

    let mut statements: Vec<StatementRelations> = Vec::new();

    for statement in ast {
        let source = statement.to_string() + ";";
        if verbose {
            println!("Source: {}", source);
        }
        match statement {
            Statement::CreateTable(CreateTable {
                name, constraints, ..
            }) => {

                let mut stmt = StatementRelations {
                    statement: source,
                    // name can be `database_name.table_name`
                    // last identifier should be the actual table name
                    name: name.0.last().unwrap().value.to_string(),
                    related_statements: Vec::new(),
                };
                for constraint in constraints {
                    if let sqlparser::ast::TableConstraint::ForeignKey { foreign_table, .. } =
                        constraint
                    {
                        foreign_table.0.iter().for_each(|s| {
                            stmt.related_statements.push(s.value.to_string());
                        });
                    }
                }
                statements.push(stmt);
            }
            Statement::CreateIndex(CreateIndex {
                name, table_name, ..
            }) => {
                if name.is_none() {
                    panic!("name is required for CREATE INDEX statement: {}", source);
                }
                let mut stmt = StatementRelations {
                    statement: source,
                    name: name.unwrap().to_string(),
                    related_statements: Vec::new(),
                };
                stmt.related_statements.push(table_name.to_string());
                statements.push(stmt);
            }
            _ => {
                panic!("This statement is not supported: {:?}", statement);
            }
        }
    }

    return statements;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_extract_statements_and_relations {
        use super::*;

        #[test]
        fn should_extract_all_statements() {
            let sql = r#"
                CREATE TABLE users (
                    id INT PRIMARY KEY,
                    name TEXT NOT NULL
                );
                CREATE TABLE posts (
                    id INT PRIMARY KEY,
                    user_id INT,
                    FOREIGN KEY (user_id) REFERENCES users (id)
                );
                CREATE INDEX posts_user_id ON posts (user_id);
            "#;

            let statements = extract_statements_and_relations(sql.to_string(), false);
            assert_eq!(statements.len(), 3);
        }

        #[test]
        fn extracted_statement_should_be_transformed_to_oneline() {
            let sql = r#"
                CREATE TABLE users (
                    id INT PRIMARY KEY,
                    name TEXT NOT NULL
                );
            "#;

            let statements = extract_statements_and_relations(sql.to_string(), false);
            assert_eq!(
                statements[0].statement,
                "CREATE TABLE users (id INT PRIMARY KEY, name TEXT NOT NULL);"
            );
        }

        #[test]
        fn should_contain_foreign_key_relations_for_table_and_index() {
            let sql = r#"
                CREATE TABLE users (
                    id INT PRIMARY KEY,
                    name TEXT NOT NULL
                );
                CREATE TABLE posts (
                    id INT PRIMARY KEY,
                    user_id INT,
                    FOREIGN KEY (user_id) REFERENCES users (id)
                );
                CREATE INDEX posts_user_id ON posts (user_id);
            "#;

            let statements = extract_statements_and_relations(sql.to_string(), false);

            assert_eq!(statements[0].related_statements.len(), 0);
            assert_eq!(statements[1].related_statements.len(), 1);
            assert_eq!(statements[1].related_statements[0], "users");
            assert_eq!(statements[2].related_statements.len(), 1);
            assert_eq!(statements[2].related_statements[0], "posts");
        }

        #[test]
        fn should_respect_comment_with_eq() {
            let sql = r#"
                CREATE TABLE users (
                    id INT PRIMARY KEY,
                    name TEXT NOT NULL
                ) COMMENT = 'This is a table comment';
            "#;

            let statements = extract_statements_and_relations(sql.to_string(), false);
            assert_eq!(
                statements[0].statement,
                "CREATE TABLE users (id INT PRIMARY KEY, name TEXT NOT NULL) COMMENT = 'This is a table comment';"
            );
        }

        #[test]
        fn should_expect_comment_without_eq() {
            let sql = r#"
                CREATE TABLE users (
                    id INT PRIMARY KEY,
                    name TEXT NOT NULL
                ) COMMENT 'This is a table comment';
            "#;

            let statements = extract_statements_and_relations(sql.to_string(), false);
            assert_eq!(
                statements[0].statement,
                "CREATE TABLE users (id INT PRIMARY KEY, name TEXT NOT NULL) COMMENT 'This is a table comment';"
            );
        }
    }
}
