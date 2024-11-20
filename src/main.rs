use sql_query_parser::parse_sql;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: sql_query_parser <command> [options]");
        println!("Commands:");
        println!("  parse <file>  - Parse a file containing SQL queries.");
        println!("  help          - Show this help message.");
        println!("  credits       - Show project credits.");
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: No file provided.");
                return;
            }
            let content = fs::read_to_string(&args[2]).expect("Failed to read the file");
            match parse_sql(&content) {
                Ok(ast) => println!("Parsed successfully: {:?}", ast),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        "help" => {
            println!("SQL Query Parser CLI Tool");
            println!("Usage: sql_query_parser <command> [options]");
        }
        "credits" => {
            println!("SQL Query Parser developed by Svirina Daria.");
        }
        _ => eprintln!("Unknown command. Use 'help' for usage information."),
    }
}
