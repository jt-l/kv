extern crate clap;

use clap::{App, Arg, SubCommand};

use std::process;

use pkvstore::{Result, PkvStore};

use tempfile::TempDir;

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

    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = PkvStore::open(temp_dir.path())?;

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            if let Some(key) = _matches.value_of("key") {
                if let Some(value) = _matches.value_of("value") {
                    store.set(key.to_string(), value.to_string());
                }
            }
        }

        ("get", Some(_matches)) => {
            if let Some(key) = _matches.value_of("key") {
                store.get(key.to_string());
            }
        }

        ("rm", Some(_matches)) => {
            if let Some(key) = _matches.value_of("key") {
                store.remove(key.to_string());
            }
        }             

        _ => unreachable!(),
    }

    Ok(())
}
