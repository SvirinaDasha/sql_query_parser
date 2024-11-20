use sql_query_parser::parse_sql;

#[test]
fn test_valid_select_with_where() {
    let input = "SELECT id, name FROM users WHERE age > 18";
    assert!(parse_sql(input).is_ok());
}

#[test]
fn test_valid_insert_single_row() {
    let input = "INSERT INTO users VALUES (1, \"John\")";
    assert!(parse_sql(input).is_ok());
}

#[test]
fn test_valid_update_with_where() {
    let input = "UPDATE users SET name = \"John\" WHERE id = 1";
    assert!(parse_sql(input).is_ok());
}

#[test]
fn test_valid_delete_with_where() {
    let input = "DELETE FROM users WHERE id = 1";
    assert!(parse_sql(input).is_ok());
}

#[test]
fn test_invalid_syntax_empty_input() {
    let input = "";
    assert!(parse_sql(input).is_err());
}

