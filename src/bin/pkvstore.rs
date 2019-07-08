extern crate clap;

use clap::{App, Arg, SubCommand};

use std::process;

use pkvstore::{Result};

fn main() -> Result<()> {

    let matches = App::new("pkvstore")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of a key from the store")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The name of the key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a value for a given key")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The name of the key"),
                )
                .arg(
                    Arg::with_name("value")
                        .index(2)
                        .required(true)
                        .help("The value you want to set"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a key and its value from the store")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The key that you want to remove"),
                ),
        )
        .get_matches();

    Ok(())

}
