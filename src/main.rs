mod query_context;
mod configuration;
mod log_file;


#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate clap;
extern crate regex;
extern crate log_ql;
use clap::{Arg, App};

fn main() {
    let matches = App::new("diagnostic")
                    .version("1.0")
                    .author("Jan Schulte. <schulte@unexpected-co.de>")
                    .about("Provides insights about your log files")
                    .arg(Arg::with_name("working_directory")
                            .short("w")
                            .long("working_directory")
                            .value_name("DIRECTORY")
                            .help("specifies working directory")
                            .takes_value(true))
                    .arg(Arg::with_name("QUERY")
                               .help("Query to execute")
                               .required(true)
                               .index(1))
                    .get_matches();
    let working_directory = matches.value_of("working_directory").unwrap_or("./");
    let query = matches.value_of("QUERY").unwrap();

    let configuration = configuration::Configuration::from_file("config.json");

    let query_context = match query_context::QueryContext::new(working_directory, configuration) {
        Ok(context) => context,
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1)
        }
    };

    match query_context.execute_query(query.into()) {
        Ok(result_list) => {
            for row in &result_list {
                println!("| {} |", row);
            }
            println!("---");
            println!("{} rows returned", result_list.len());
        },
        Err(error) => {
            println!("ERROR: {}", error);
            std::process::exit(1)
        }
    }
}
