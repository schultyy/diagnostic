mod query_context;

extern crate clap;
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


    let query_context = match query_context::QueryContext::new(working_directory) {
        Ok(context) => context,
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1)
        }
    };

    match query_context.execute_query(query.into()) {
        Ok(result) => {
            println!("{}", result);
        },
        Err(error) => {
            println!("{}", error);
            std::process::exit(1)
        }
    }
}
