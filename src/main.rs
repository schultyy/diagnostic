extern crate clap;
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
                    .get_matches();
    let working_directory = matches.value_of("working_directory").unwrap_or("./");
    println!("Value for config: {}", working_directory);
}
