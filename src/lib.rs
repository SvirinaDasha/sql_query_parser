use pest::Parser;
use thiserror::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SQLParser;

#[derive(Debug)]
pub struct AST {
    pub query: String,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing failed: {0}")]
    PestError(String),
}

pub fn parse_sql(input: &str) -> Result<AST, ParseError> {
    let parsed = SQLParser::parse(Rule::query, input)
        .map_err(|e| ParseError::PestError(e.to_string()))?;

    Ok(AST {
        query: format!("{:?}", parsed),
    })
}
