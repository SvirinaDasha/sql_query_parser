# SQL Query Parser

## Project Description

**SQL Query Parser** is a lightweight Rust-based tool designed to parse and validate basic SQL queries. The parser supports SQL commands such as `SELECT`, `INSERT`, `UPDATE`, and `DELETE`, providing functionality to analyze, validate, and potentially optimize or transform SQL queries. The goal is to simplify query processing and ensure syntax correctness for integration into larger systems.

---

## Technical Description

### Core Components

- **Grammar (`grammar.pest`)**: Defines the parsing rules for supported SQL commands.
- **Parser Logic (`lib.rs`)**: Implements parsing functionality and constructs an abstract syntax tree (AST).
- **Command-Line Interface (`main.rs`)**: Allows interaction with the parser through CLI commands.

### Grammar and Parsing

The parsing process is based on a structured grammar defined in `grammar.pest`. It supports SQL commands such as:

- **SELECT**: Retrieves data from a table, with optional `WHERE` clauses for filtering.
- **INSERT**: Adds new rows to a table with specified values.
- **UPDATE**: Modifies existing rows in a table with optional conditions.
- **DELETE**: Removes rows from a table, optionally filtered by a condition.

Below is an excerpt from the grammar file:

```pest
query = _{ select | insert | update | delete }

select = { "SELECT" ~ columns ~ "FROM" ~ table ~ (where_clause)? }
insert = { "INSERT INTO" ~ table ~ "VALUES" ~ "(" ~ values ~ ")" }
update = { "UPDATE" ~ table ~ "SET" ~ assignments ~ (where_clause)? }
delete = { "DELETE FROM" ~ table ~ (where_clause)? }

columns = @{ identifier ~ ("," ~ identifier)* }
values = @{ literal ~ ("," ~ literal)* }
assignments = { identifier ~ "=" ~ literal ~ ("," ~ identifier ~ "=" ~ literal)* }
table = @{ identifier }
where_clause = { "WHERE" ~ condition }
condition = { identifier ~ operator ~ literal }
operator = _{ "=" | "!=" | ">" | "<" }
identifier = @{ ASCII_ALPHANUMERIC+ | "_" }
literal = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" | ASCII_DIGIT+ }


## How to Use

### Example File for Parsing (`test_parser.txt`)

```sql
SELECT id, name FROM users WHERE age > 18;
INSERT INTO users VALUES (1, "John Doe"), (2, "Jane Doe");
UPDATE users SET name = "John Smith" WHERE id = 1;
DELETE FROM users WHERE age < 18;

Running the Parser
To parse a file containing SQL queries, use the following command:
cargo run -- parse test_parser.txt
Expected Output

Parsed successfully: AST { query: "[Pair { rule: select, ... }]" }


CLI Commands
parse <file>: Parses SQL queries from the specified file.
help: Displays help information about available commands.
credits: Provides details about the project and the author.

Author
Author: Svirina Daria
License: MIT



