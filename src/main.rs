use clap::{App, Arg};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize)]
struct TemplateVariables {
    #[serde(flatten)]
    variables: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Template Substitution")
        .version("0.1")
        .author("Sivaram Sajeev")
        .about("Substitutes variables in a template file")
        .arg(
            Arg::with_name("template")
                .short("t")
                .long("template")
                .value_name("TEMPLATE")
                .help("Sets the template file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("variables")
                .short("v")
                .long("variables")
                .value_name("VARIABLES")
                .help("Sets the variables file in YAML format")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Sets the output file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let template_file = matches.value_of("template").unwrap();
    let variables_file = matches.value_of("variables").unwrap();
    let output_file = matches.value_of("output").unwrap();

    let mut template_file = File::open(template_file)?;
    let mut template_content = String::new();
    template_file.read_to_string(&mut template_content)?;

    let mut variables_file = File::open(variables_file)?;
    let mut variables_content = String::new();
    variables_file.read_to_string(&mut variables_content)?;

    let variables: TemplateVariables = serde_yaml::from_str(&variables_content)?;

    let substituted_content = substitute_variables(&template_content, &variables.variables);

    let mut output_file = File::create(output_file)?;
    output_file.write_all(substituted_content.as_bytes())?;

    Ok(())
}

fn substitute_variables(template_content: &str, variables: &HashMap<String, String>) -> String {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .render_template(template_content, variables)
        .unwrap()
}
