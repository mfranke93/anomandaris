use std::fs;
use pest::Parser;
use pest_derive::Parser;

mod evaluate;

const CONFIG_FILE_NAME: &'static str = "Makefile";

#[derive(Parser)]
#[grammar = "makefile.pest"]
pub struct MakefileParser;

fn main() {
    let file_path = CONFIG_FILE_NAME;
    let error_msg = format!("Could not read {file_path}");
    let contents = fs::read_to_string(file_path).expect(&error_msg);

    let successful_parse = MakefileParser::parse(Rule::Grammar, contents.as_str()).expect("grammar error").next().unwrap();
    println!("{:#?}\n\n---\n\n", successful_parse);

    for val in successful_parse.clone().into_inner() {
        match val.as_rule() {
            Rule::EmptyLine => {
                println!("EMPTY LINE");
            },
            Rule::Comment => {
                let comment = val.as_str();
                println!("COMMENT   {comment}");
            },
            Rule::Declaration => {
                let mut inner_rules = val.into_inner();
                let name = inner_rules
                    .next()
                    .unwrap()
                    .as_str();
                let value = inner_rules.next().unwrap().as_str();
                println!("DECLARATION   `{name}` -> `{value}`")
            },
            Rule::Rule => {
                let mut inner_rules = val.into_inner();
                let targets = inner_rules
                    .next()
                    .unwrap()
                    .as_str();
                let dependencies = inner_rules.next().unwrap().as_str();
                println!("RULE   `{targets}` (depends on `{dependencies}`)");

                let recipe_lines = inner_rules.next().unwrap();
                for line in recipe_lines.into_inner() {
                    match line.as_rule() {
                        Rule::RuleRecipeLine => {
                            let recipe_line = line.as_str();
                            println!(" - | {recipe_line}")
                        },
                        _ => {},
                    }
                }
            }
            _ => {},
        }
    }

    let vars = evaluate::resolve_variables(successful_parse.clone());
    match vars {
        Some(variables) => {
            println!("\nVariables:\n");
            for (k, v) in variables.iter() {
                println!(" {} = `{}`", k, v);
            }
        },
        None => {},
    }
}
