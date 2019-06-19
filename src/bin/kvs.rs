use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;

fn main(){
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("Sets a file")
             .takes_value(true))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let file = matches.value_of("file");
    let mut kv = kvs::KvStore::new(file);
    
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            kv.set(_matches.value_of("KEY").unwrap().to_string(),
                   _matches.value_of("VALUE").unwrap().to_string());
        }
        ("get", Some(_matches)) => {
            match kv.get(_matches.value_of("KEY").unwrap().to_string()) {
                Some(x) =>  {println!("{}", x)}
                _ => {exit(0)}
            }
        }
        ("rm", Some(_matches)) => {
            
            kv.remove(_matches.value_of("KEY").unwrap().to_string());
        }
        _ =>exit(0),
    }
    exit(0);
}
