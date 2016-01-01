extern crate clap;
extern crate rand;
extern crate toml;
extern crate serde;

mod error;
mod group;
mod hero;
mod matchmaker;
mod mode;
mod plan;
mod player;

use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use clap::{App, AppSettings, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use toml::{Encoder, Decoder, Parser, Value};

use error::Error;
use matchmaker::Matchmaker;
use mode::Mode;
use plan::Plan;

fn main() {
    if let Err(error) = real_main() {
        println!("Error: {}", error);
        exit(1);
    }
}

fn real_main() -> Result<(), Error> {
    let app_matches = App::new("hotsq")
        .about("A matchmaking system simulator for Heroes of the Storm")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generates a plan file")
                .arg(
                    Arg::with_name("path")
                        .help("Path where the plan file should be written")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("count")
                        .help("The number of players to add to the queue (default: 100)")
                        .short("c")
                        .long("count")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the matchmaker on a plan file")
                .arg(
                    Arg::with_name("path")
                        .help("Path to a plan file")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    match app_matches.subcommand() {
        ("generate", Some(matches)) => {
            let count = match matches.value_of("count") {
                Some(string_count) => try!(string_count.parse::<u64>()),
                None => 100,
            };
            let plan_path = matches.value_of("path").expect("plan path was not supplied");

            let plan = try!(Plan::generate(Mode::QuickMatch, count));
            let mut file = try!(File::create(plan_path));
            let mut encoder = Encoder::new();
            if let Err(error) = plan.serialize(&mut encoder) {
                println!("{:?}", plan);
                return Err(Error::new(format!("{}", error)));
            }

            match write!(file, "{}", Value::Table(encoder.toml)) {
                Ok(_) => Ok(()),
                Err(error) => Err(From::from(error)),
            }
        },
        ("run", Some(matches)) => {
            let plan_path = matches.value_of("path").expect("plan path was not supplied");
            let mut plan_file = try!(File::open(plan_path));
            let mut contents = String::new();
            try!(plan_file.read_to_string(&mut contents));
            let mut parser = Parser::new(&contents);
            let toml_value = match parser.parse() {
                Some(value) => Value::Table(value),
                None => return Err(::std::convert::From::from(&parser.errors[0])),
            };
            let mut decoder = Decoder::new(toml_value);
            let plan: Plan = try!(Deserialize::deserialize(&mut decoder));
            let matchmaker = Matchmaker::new(&plan);

            matchmaker.run();

            Ok(())
        },
        _ => unreachable!(),
    }
}
