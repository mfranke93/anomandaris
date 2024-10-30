use std::{collections::HashMap, io::Error};
use pest::iterators::{Pair, Pairs};
use crate::Rule;

pub fn resolve_variables(
    parsed_file: Pair<'_, Rule>
) -> Option<HashMap<String, String>> {
    let mut variable_values = HashMap::new();

    for line in parsed_file.into_inner() {
        match line.as_rule() {
            Rule::Declaration => {
                let mut inner_rules = line.into_inner();
                let name = inner_rules
                    .next()
                    .unwrap()
                    .as_str();
                let value = inner_rules.next().unwrap();
                let value_value = evaluate_inner(&variable_values, value.clone());

                match variable_values.insert(name.to_string(), value_value.clone()) {
                    Some(old_value) => println!("WARN: updated the value of `{}`: \"{}\" -> \"{}\"", name, old_value, value_value),
                    None => {},
                }
            },
            _ => {},
        }
    }

    Some(variable_values)
}


fn evaluate_inner(
    known_variables: &HashMap<String, String>,
    value: Pair<'_, Rule>,
) -> String {
    let mut evaluated_content = String::new();

    for chunk in value.into_inner() {
        match chunk.as_rule() {
            Rule::NonStatement => {
                evaluated_content += chunk.as_str();
            },
            Rule::LiteralDollar => {
                evaluated_content += "$";
            },
            Rule::EvaluatedStatement => {
                let variable_name = chunk.into_inner().next().unwrap().as_str();
                match known_variables.get(variable_name) {
                    Some(value) => evaluated_content += value.as_str(),
                    None => panic!("Undefined variable `{:?}`", variable_name),
                }
            },
            _ => {
                panic!("This should not happen!\n{:#?}", chunk);
            }
        }
    }
    
    evaluated_content
}