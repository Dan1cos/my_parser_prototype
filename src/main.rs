use anyhow::Result;
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

pub fn main() -> Result<()>{
    let parsed_data = Grammar::parse(Rule::field, "123")?;
    println!("{:?}", parsed_data);

    Ok(())
}